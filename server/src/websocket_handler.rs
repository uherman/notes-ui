use futures_util::SinkExt;
use models::{Command, Note, WebSocketMessage};
use redis::{aio::MultiplexedConnection, AsyncCommands};
use serde_json;
use ws::stream::DuplexStream;

use crate::models;

/// Handles the `Set` command by saving a note to the Redis database.
///
/// # Arguments
///
/// * `conn` - A multiplexed Redis connection.
/// * `note` - The note to be saved.
///
/// # Returns
///
/// A JSON string indicating the result of the operation.
pub async fn handle_set_command(mut conn: MultiplexedConnection, note: Note) -> String {
    let serialized_note = serde_json::to_string(&note).unwrap();

    conn.set::<&str, String, ()>(format!("note:{}", &note.id).as_str(), serialized_note)
        .await
        .unwrap();

    serde_json::to_string(&models::WebSocketResponse {
        response: 200,
        message: Some("Note saved".to_string()),
    })
    .unwrap()
}

/// Handles the `Delete` command by removing a note from the Redis database.
///
/// # Arguments
///
/// * `conn` - A multiplexed Redis connection.
/// * `id` - The ID of the note to be deleted.
///
/// # Returns
///
/// A JSON string indicating the result of the operation.
pub async fn handle_delete_command(mut conn: MultiplexedConnection, id: String) -> String {
    conn.del::<&str, i16>(format!("note:{}", &id).as_str())
        .await
        .unwrap();

    serde_json::to_string(&models::WebSocketResponse {
        response: 200,
        message: Some("Note deleted".to_string()),
    })
    .unwrap()
}

/// Handles the `Get` command by retrieving all notes from the Redis database.
///
/// # Arguments
///
/// * `conn` - A multiplexed Redis connection.
///
/// # Returns
///
/// A JSON string containing all notes.
pub async fn handle_get_command(mut conn: MultiplexedConnection) -> String {
    let keys: Vec<String> = conn.keys::<&str, Vec<String>>("note:*").await.unwrap();

    let mut notes = Vec::new();
    for key in keys {
        let note: String = conn.get::<&str, String>(&key).await.unwrap();
        notes.push(serde_json::from_str::<Note>(&note).unwrap());
    }

    serde_json::to_string(&notes).unwrap()
}

/// Handles incoming WebSocket messages and dispatches them to the appropriate command handler.
///
/// # Arguments
///
/// * `stream` - The WebSocket stream.
/// * `text` - The incoming message text.
/// * `redis_conn` - A multiplexed Redis connection.
pub async fn handle_websocket_message(
    stream: &mut DuplexStream,
    text: String,
    redis_conn: MultiplexedConnection,
) {
    match serde_json::from_str::<WebSocketMessage>(&text) {
        Ok(web_socket_message) => match web_socket_message.command {
            Command::Get => {
                let response = handle_get_command(redis_conn).await;
                stream.send(ws::Message::Text(response)).await.unwrap();
            }
            Command::Set => match web_socket_message.note {
                Some(note) => {
                    let response = handle_set_command(redis_conn, note).await;
                    stream.send(ws::Message::Text(response)).await.unwrap();
                }
                None => {
                    stream
                        .send(ws::Message::Text(
                            serde_json::to_string(&models::WebSocketResponse {
                                response: 400,
                                message: Some("Note is required".to_string()),
                            })
                            .unwrap(),
                        ))
                        .await
                        .unwrap();
                }
            },
            Command::Delete => match web_socket_message.note {
                Some(note) => {
                    let response = handle_delete_command(redis_conn, note.id).await;
                    stream.send(ws::Message::Text(response)).await.unwrap();
                }
                None => {
                    stream
                        .send(ws::Message::Text(
                            serde_json::to_string(&models::WebSocketResponse {
                                response: 400,
                                message: Some("Note is required".to_string()),
                            })
                            .unwrap(),
                        ))
                        .await
                        .unwrap();
                }
            },
        },
        Err(_) => {
            stream
                .send(ws::Message::Text(
                    serde_json::to_string(&models::WebSocketResponse {
                        response: 400,
                        message: Some("Invalid message".to_string()),
                    })
                    .unwrap(),
                ))
                .await
                .unwrap();
        }
    }
}
