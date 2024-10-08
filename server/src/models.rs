use serde::{Deserialize, Serialize};

/// Represents a note with an id, content, and updated timestamp.
///
/// # Fields
///
/// * `id` - A unique identifier for the note.
/// * `content` - The content of the note.
/// * `updated` - The timestamp when the note was last updated.
#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub id: String,
    pub content: Option<String>,
    pub updated: Option<String>,
}

/// Enum representing possible commands that can be sent over WebSocket.
///
/// # Variants
///
/// * `Get` - Command to retrieve notes.
/// * `Set` - Command to set a note.
/// * `Delete` - Command to delete a note.
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Get,
    Set,
    Delete,
}

/// Represents a WebSocket message containing a command and an optional note.
///
/// # Fields
///
/// * `command` - The command to be executed.
/// * `note` - An optional note associated with the command.
#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketMessage {
    pub command: Command,
    pub note: Option<Note>,
}

/// Represents a response to be sent over WebSocket.
///
/// # Fields
///
/// * `response` - The response code indicating the result of the command.
#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketResponse {
    pub response: u16,
    pub message: Option<String>,
}
