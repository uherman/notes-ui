mod commands;
mod models;

#[macro_use]
extern crate log;
extern crate dotenv;

use commands::{handle_delete_command, handle_get_command, handle_set_command};
use dotenv::dotenv;
use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use models::{Command, WebSocketMessage, WebSocketResponse};
use redis::aio::MultiplexedConnection;
use std::{env, sync::Arc};
use tokio::sync::Mutex;
use warp::{
    filters::ws::{Message, WebSocket},
    Filter,
};

/// Entry point for the application.
///
/// Initializes the logger, sets up the Redis connection, and starts the WebSocket server.
#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    warp::serve(setup_routes(init_redis_connection().await))
        .run(([127, 0, 0, 1], 5000))
        .await;
}

/// Initializes the Redis connection.
///
/// # Returns
/// An `Arc<Mutex<MultiplexedConnection>>` representing the Redis connection.
async fn init_redis_connection() -> Arc<Mutex<MultiplexedConnection>> {
    let redis_client = redis::Client::open(env::var("REDIS_URL").unwrap()).unwrap();
    Arc::new(Mutex::new(
        redis_client
            .get_multiplexed_async_connection()
            .await
            .unwrap(),
    ))
}

/// Sets up the WebSocket routes.
///
/// # Parameters
/// - `redis_conn`: An `Arc<Mutex<MultiplexedConnection>>` representing the Redis connection.
///
/// # Returns
/// A `warp::Filter` that handles WebSocket connections.
fn setup_routes(
    redis_conn: Arc<Mutex<MultiplexedConnection>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let redis_conn = redis_conn.clone();
            ws.on_upgrade(move |socket| connected(socket, redis_conn))
        });

    let root_route = warp::path::end().map(|| {
        warp::reply::html(
            "<body style='display:flex;align-items:center;justify-content:center;height:100vh;background:#1e2030;'><h1 style='text-center'>📝</h1></body>",
        )
    });

    ws_route.or(root_route).boxed()
}

/// Handles a new WebSocket connection.
///
/// # Parameters
/// - `ws`: The WebSocket connection.
/// - `redis_conn`: An `Arc<Mutex<MultiplexedConnection>>` representing the Redis connection.
async fn connected(ws: WebSocket, redis_conn: Arc<Mutex<MultiplexedConnection>>) {
    info!("Client connected");
    let (mut ws_tx, mut ws_rx) = ws.split();

    while let Some(msg) = ws_rx.next().await {
        if let Ok(msg) = msg {
            if msg.is_text() {
                handle_message(msg.to_str().unwrap(), &mut ws_tx, redis_conn.clone()).await;
            } else {
                send_error_response(&mut ws_tx, 400, None).await;
            }
        }
    }
}

/// Handles an incoming WebSocket message.
///
/// # Parameters
/// - `msg`: The message received from the WebSocket.
/// - `ws_tx`: The WebSocket sender.
/// - `redis_conn`: An `Arc<Mutex<MultiplexedConnection>>` representing the Redis connection.
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
                    send_error_response(ws_tx, 400, Some("Note is required")).await;
                }
            }
            Command::Delete => {
                if let Some(note) = message.note {
                    if let Err(e) = handle_delete_command(ws_tx, redis_conn.clone(), note.id).await
                    {
                        error!("Error while handling DELETE command: {}", e);
                    }
                } else {
                    send_error_response(ws_tx, 400, Some("Note is required")).await;
                }
            }
        },
        Err(e) => {
            error!("Failed to deserialize WebSocketMessage: {}", e);
            send_error_response(ws_tx, 400, Some(e.to_string().as_str())).await;
        }
    }
}

/// Sends an error response over the WebSocket.
///
/// # Parameters
/// - `ws_tx`: The WebSocket sender.
/// - `code`: The error code to send.
async fn send_error_response(
    ws_tx: &mut SplitSink<WebSocket, Message>,
    code: u16,
    message: Option<&str>,
) {
    let error_response = serde_json::to_string(&WebSocketResponse {
        response: code,
        message: message.map(|s| s.to_string()),
    })
    .unwrap();
    ws_tx
        .send(warp::ws::Message::text(error_response))
        .await
        .unwrap();
}
