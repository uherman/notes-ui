mod commands;
mod models;

use commands::{handle_delete_command, handle_get_command, handle_set_command};
use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use models::{Command, WebSocketMessage, WebSocketResponse};
use redis::aio::MultiplexedConnection;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{
    filters::ws::{Message, WebSocket},
    Filter,
};

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    warp::serve(setup_routes(init_redis_connection().await))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn init_redis_connection() -> Arc<Mutex<MultiplexedConnection>> {
    let redis_client = redis::Client::open("redis://127.0.0.1/").unwrap();
    Arc::new(Mutex::new(
        redis_client
            .get_multiplexed_async_connection()
            .await
            .unwrap(),
    ))
}

fn setup_routes(
    redis_conn: Arc<Mutex<MultiplexedConnection>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("echo")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let redis_conn = redis_conn.clone();
            ws.on_upgrade(move |socket| connected(socket, redis_conn))
        })
}

async fn connected(ws: WebSocket, redis_conn: Arc<Mutex<MultiplexedConnection>>) {
    info!("Client connected");
    let (mut ws_tx, mut ws_rx) = ws.split();

    while let Some(msg) = ws_rx.next().await {
        if let Ok(msg) = msg {
            if msg.is_text() {
                handle_message(msg.to_str().unwrap(), &mut ws_tx, redis_conn.clone()).await;
            } else {
                send_error_response(&mut ws_tx, 400).await;
            }
        }
    }
}

async fn handle_message(
    msg: &str,
    ws_tx: &mut SplitSink<WebSocket, Message>,
    redis_conn: Arc<Mutex<MultiplexedConnection>>,
) {
    match serde_json::from_str::<WebSocketMessage>(msg) {
        Ok(message) => match message.command {
            Command::Get => {
                if let Err(e) = handle_get_command(ws_tx, redis_conn.clone()).await {
                    error!("Error while handling GET command: {}", e);
                }
            }
            Command::Set => {
                if let Some(note) = message.note {
                    if let Err(e) = handle_set_command(ws_tx, redis_conn.clone(), note).await {
                        error!("Error while handling SET command: {}", e);
                    }
                } else {
                    send_error_response(ws_tx, 400).await;
                }
            }
            Command::Delete => {
                if let Some(note) = message.note {
                    if let Err(e) = handle_delete_command(ws_tx, redis_conn.clone(), note.id).await
                    {
                        error!("Error while handling DELETE command: {}", e);
                    }
                } else {
                    send_error_response(ws_tx, 400).await;
                }
            }
        },
        Err(e) => {
            info!("Failed to deserialize WebSocketMessage: {}", e);
            send_error_response(ws_tx, 400).await;
        }
    }
}

async fn send_error_response(ws_tx: &mut SplitSink<WebSocket, Message>, code: u16) {
    let error_response = serde_json::to_string(&WebSocketResponse { response: code }).unwrap();
    ws_tx
        .send(warp::ws::Message::text(error_response))
        .await
        .unwrap();
}
