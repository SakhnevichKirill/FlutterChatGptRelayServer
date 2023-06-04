use chatgpt::types::ChatMessage;
use chatgpt::types::Role;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use ws_server::models::ClientReq;

#[tokio::main]
async fn main() {
    // Spawn a single client.
    // Try to get a stream that is opened between a client and a server.
    let ws_stream = match connect_async("ws://0.0.0.0:8080/ws").await {
        // In case Ok is called, the handshake has been successful.
        Ok((stream, response)) => {
            println!("Handshake has been completed successfully");
            stream
        }
        Err(e) => {
            println!("A handshake failed with error {e}");
            return;
        }
    };

    // Split the stream into sender and receiver.
    let (mut sender, mut receiver) = ws_stream.split();

    let history: Vec<ChatMessage> = Vec::new();

    // Send multiple messages to the server.
    sender
        .send(Message::Text(
            serde_json::to_string(&ClientReq {
                new_message: ChatMessage {
                    role: Role::User,
                    content: String::from("What are you?"),
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
            // Check if the server wants to interrupt the connection.
            if msg == "stop" {
                // Inform the user that the connection is going to be closed.
                println!("Got \"stop\" message from server");
                println!("Closing the connection...");
                sender.close().await.unwrap();
                println!("The connection is closed");
            }
            // Print this text.
            println!("Got the message from the server: {msg}");
        }
    }
}
