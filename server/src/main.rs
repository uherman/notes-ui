mod commands;
mod models;

use commands::{handle_delete_command, handle_get_command, handle_set_command};
use futures_util::{SinkExt, StreamExt};
use models::{Command, WebSocketMessage, WebSocketResponse};
use redis::aio::MultiplexedConnection;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{filters::ws::WebSocket, Filter};

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let redis_client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let redis_conn = Arc::new(Mutex::new(
        redis_client
            .get_multiplexed_async_connection()
            .await
            .unwrap(),
    ));

    let routes = warp::path("echo")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let redis_conn = redis_conn.clone();
            ws.on_upgrade(move |socket| connected(socket, redis_conn))
        });

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn connected(ws: WebSocket, redis_conn: Arc<Mutex<MultiplexedConnection>>) {
    info!("Client connected");
    let (mut ws_tx, mut ws_rx) = ws.split();

    while let Some(msg) = ws_rx.next().await {
        let msg = msg.unwrap();
        if msg.is_text() {
            let msg = msg.to_str().unwrap().to_string();
            match serde_json::from_str::<WebSocketMessage>(&msg) {
                Ok(message) => match message.command {
                    Command::Get => {
                        if let Err(e) = handle_get_command(&mut ws_tx, redis_conn.clone()).await {
                            error!("Error while handling GET command: {}", e);
                        }
                    }
                    Command::Set => {
                        if let Some(note) = message.note {
                            if let Err(e) =
                                handle_set_command(&mut ws_tx, redis_conn.clone(), note).await
                            {
                                error!("Error while handling SET command: {}", e);
                            }
                        } else {
                            error!("SET command received without a note");
                            let error_response =
                                serde_json::to_string(&WebSocketResponse { response: 400 })
                                    .unwrap();
                            ws_tx
                                .send(warp::ws::Message::text(error_response))
                                .await
                                .unwrap();
                        }
                    }
                    Command::Delete => {
                        if let Some(note) = message.note {
                            if let Err(e) =
                                handle_delete_command(&mut ws_tx, redis_conn.clone(), note.id).await
                            {
                                error!("Error while handling DELETE command: {}", e);
                            }
                        } else {
                            error!("DELETE command received without a note");
                            let error_response =
                                serde_json::to_string(&WebSocketResponse { response: 400 })
                                    .unwrap();
                            ws_tx
                                .send(warp::ws::Message::text(error_response))
                                .await
                                .unwrap();
                        }
                    }
                },
                Err(e) => {
                    info!("Failed to deserialize WebSocketMessage: {}", e);
                    let response =
                        serde_json::to_string(&WebSocketResponse { response: 400 }).unwrap();

                    ws_tx.send(warp::ws::Message::text(response)).await.unwrap();
                }
            }
        }
    }
}
