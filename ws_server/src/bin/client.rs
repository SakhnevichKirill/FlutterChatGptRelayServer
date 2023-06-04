//! This client is meant for testing purposes of the server.
//!
//! To use it:
//!
//! 1. Start the main server using using one of the following options:
//!     a. Using docker-compose
//!     b. Using `cargo run --bin client` command.
//!
//! 2. Run this client and check if it manages to get a response from ChatGPT.

use chatgpt::types::ChatMessage;
use chatgpt::types::Role;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use ws_server::models::ClientReq;

#[tokio::main]
async fn main() {
    // Spawn a single client.
    // Try to get a stream that is opened between a client and a server.
    let ws_stream = match connect_async("ws://95.165.88.39:80/ws").await {
        // In case Ok is called, the handshake has been successful.
        Ok((stream, _response)) => {
            println!("Handshake has been completed successfully");
            stream
        } // end Ok
        Err(e) => {
            println!("A handshake failed with error {e}");
            return;
        } // end Err
    }; // en let ws_stream

    // Split the stream into sender and receiver.
    let (mut sender, mut receiver) = ws_stream.split();

    let _history: Vec<ChatMessage> = Vec::new();

    // Send multiple messages to the server.
    sender
        .send(Message::Text(
            serde_json::to_string(&ClientReq {
                new_message: ChatMessage {
                    role: Role::User,
                    content: String::from("Hello!"),
                },
                history: Vec::new(),
            })
            .unwrap(),
        ))
        .await
        .expect("Failed to send a message to the server");

    // Await responses from the server.
    while let Some(Ok(msg)) = receiver.next().await {
        // Match the message.
        if let Message::Text(msg) = msg {
            // Print this text.
            println!("Got the message from the server: {msg}");
        } else if let Message::Close(_) = msg {
            // This is the end of the message, close the connection.
            println!("The message is received!");
            println!("Closing connection...");
            break;
        } // end if
    } // end while let

    println!("Client is stopped");
} // end fn main()
