use crate::models::{Note, WebSocketResponse};
use futures_util::{stream::SplitSink, SinkExt};
use redis::{aio::MultiplexedConnection, AsyncCommands};
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::filters::ws::{Message, WebSocket};

const GET_COMMAND_ERROR: &str = "An error occurred while retrieving the notes";
const SET_COMMAND_ERROR: &str = "An error occurred while saving the note";
const DELETE_COMMAND_ERROR: &str = "An error occurred while deleting the note";

/// Handles the "get" command by retrieving all keys and their associated values from Redis,
/// serializing them to JSON, and sending them over the WebSocket connection.
///
/// # Arguments
///
/// * `ws_tx` - A mutable reference to the WebSocket sender.
/// * `redis_conn` - An Arc-wrapped, Mutex-protected Redis connection.
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - An empty result on success, or an error on failure.
pub(crate) async fn handle_get_command(
    ws_tx: &mut SplitSink<WebSocket, Message>,
    redis_conn: Arc<Mutex<MultiplexedConnection>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = redis_conn.lock().await;

    let keys: Vec<String> = match conn.keys::<&str, Vec<String>>("note:*").await {
        Ok(keys) => keys,
        Err(e) => {
            send_response(ws_tx, 500, Some(GET_COMMAND_ERROR)).await;
            return Err(Box::new(e));
        }
    };

    let mut notes = Vec::new();
    for key in keys {
        let note: String = match conn.get::<&str, String>(&key).await {
            Ok(note) => note,
            Err(e) => {
                send_response(ws_tx, 500, Some(GET_COMMAND_ERROR)).await;
                return Err(Box::new(e));
            }
        };
        notes.push(note);
    }

    let serialized_notes = match serde_json::to_string(&notes) {
        Ok(sn) => sn,
        Err(e) => {
            send_response(ws_tx, 500, Some(GET_COMMAND_ERROR)).await;
            return Err(Box::new(e));
        }
    };

    match ws_tx.send(warp::ws::Message::text(serialized_notes)).await {
        Ok(it) => it,
        Err(e) => return Err(Box::new(e)),
    };

    Ok(())
}

/// Handles the "set" command by serializing a note and storing it in Redis,
/// then sending a response over the WebSocket connection.
///
/// # Arguments
///
/// * `ws_tx` - A mutable reference to the WebSocket sender.
/// * `redis_conn` - An Arc-wrapped, Mutex-protected Redis connection.
/// * `note` - The note to be stored.
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - An empty result on success, or an error on failure.
pub(crate) async fn handle_set_command(
    ws_tx: &mut SplitSink<WebSocket, Message>,
    redis_conn: Arc<Mutex<MultiplexedConnection>>,
    note: Note,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = redis_conn.lock().await;

    let serialized_note = match serde_json::to_string(&note) {
        Ok(sn) => sn,
        Err(e) => {
            send_response(ws_tx, 500, Some(SET_COMMAND_ERROR)).await;
            return Err(Box::new(e));
        }
    };

    if let Err(e) = conn
        .set::<&str, String, ()>(format!("note:{}", &note.id).as_str(), serialized_note)
        .await
    {
        send_response(ws_tx, 500, Some(SET_COMMAND_ERROR)).await;
        return Err(Box::new(e));
    }

    send_response(ws_tx, 200, None).await;
    info!("Saved note: {:?}", note.id);

    Ok(())
}

/// Handles the "delete" command by deleting a note from Redis,
/// then sending a response over the WebSocket connection.
///
/// # Arguments
///
/// * `ws_tx` - A mutable reference to the WebSocket sender.
/// * `redis_conn` - An Arc-wrapped, Mutex-protected Redis connection.
/// * `id` - The ID of the note to be deleted.
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - An empty result on success, or an error on failure.
pub(crate) async fn handle_delete_command(
    ws_tx: &mut SplitSink<WebSocket, Message>,
    redis_conn: Arc<Mutex<MultiplexedConnection>>,
    id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = redis_conn.lock().await;

    match conn.del::<&str, i16>(&id).await {
        Ok(count) => {
            if count == 0 {
                error!("Failed to delete note since it does not exist: {:?}", id);
                send_response(ws_tx, 404, Some("Note not found")).await;
                return Ok(());
            }
        }
        Err(e) => {
            send_response(ws_tx, 500, Some(DELETE_COMMAND_ERROR)).await;
            return Err(Box::new(e));
        }
    };

    send_response(ws_tx, 200, None).await;
    info!("Deleted note: {:?}", id);

    Ok(())
}

/// Sends a response over the WebSocket connection.
///
/// # Arguments
///
/// * `ws_tx` - A mutable reference to the WebSocket sender.
/// * `response` - The response code to be sent.
async fn send_response(
    ws_tx: &mut SplitSink<WebSocket, Message>,
    response: u16,
    message: Option<&str>,
) {
    let error_response = serde_json::to_string(&WebSocketResponse {
        response,
        message: message.map(|s| s.to_string()),
    })
    .unwrap();

    match ws_tx.send(warp::ws::Message::text(error_response)).await {
        Ok(_) => {}
        Err(e) => {
            error!("Failed to send error response: {}", e);
        }
    }
}
