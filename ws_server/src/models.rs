use chatgpt::prelude::*;
use serde::Deserialize;

// This structure represents the user request to the server.
#[derive(Deserialize)]
pub struct ClientReq {
    pub new_message: String,
    /// All the messages sent and received, starting with the beginning system message
    pub history: Vec<ChatMessage>,
} // end struct ClientReq
