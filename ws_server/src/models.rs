use serde::{Deserialize, Serialize};

// This structure represents the user request to the server.
#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub message: String,
} // end struct UserRequest
