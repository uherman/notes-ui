use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub id: String,
    pub content: String,
    pub updated: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Get,
    Set,
    Delete,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketMessage {
    pub command: Command,
    pub note: Option<Note>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketResponse {
    pub response: u16,
}
