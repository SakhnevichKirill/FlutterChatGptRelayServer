use chatgpt::prelude::*;
use serde::{Deserialize, Serialize};

// This structure represents the user request to the server.
#[derive(Deserialize, Serialize)]
pub struct ClientReq {
    pub new_message: ChatMessage,
    /// All the messages sent and received, starting with the beginning system message
    pub history: Vec<ChatMessage>,
} // end struct ClientReq
