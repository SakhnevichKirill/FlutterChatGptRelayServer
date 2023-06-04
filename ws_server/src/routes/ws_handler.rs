use axum::{
    extract::{
        connect_info::ConnectInfo,
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    headers::UserAgent,
    response::IntoResponse,
    TypedHeader,
};

use chatgpt::{prelude::*, types::Role, Result};
use chrono::Local;
use std::io::stdout;

use dotenvy::dotenv;
use futures_util::{stream::SplitSink, SinkExt, Stream, StreamExt};
use std::env;
use std::net::SocketAddr;

use crate::models::ClientReq;

// This function gives a handshake to the client that wants to open
// a websocket connection.
//
// WARNING: This is the last place where it is still possible to gather
// some information about a user like IP address and User Agent.
//
// This function still works with HTTP protocol and if everything is
// fine, opens a websocket connection with a client.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    _user_agent: Option<TypedHeader<UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    println!(
        "A user with IP: {} wants to establish websocket connection",
        addr.ip()
    );
    // Move from HTTP protocol to websocket.
    ws.on_upgrade(move |socket| socket_handler(socket, addr))
} // end fn ws_handler()

// This function works with already established websocket connections.
async fn socket_handler(socket: WebSocket, addr: SocketAddr) {
    println!("A user with IP: {} is connected to a websocket", addr.ip());

    // Split receiver and producer.
    let (mut sender, mut receiver) = socket.split();

    // Current task.
    let mut cur_task: Option<tokio::task::JoinHandle<()>> = None;

    // Await requests from the user.
    while let Some(Ok(msg)) = receiver.next().await {
        // Check what type the message is.
        match msg {
            Message::Text(msg) => {
                // This is a text request to ChatGPT.

                // Print out the user's request.
                println!("Got request: {}", msg);

                // Deserialize the request.
                let ClientReq {
                    new_message,
                    history,
                } = serde_json::from_str(&msg).expect("Failed to convert the passed json");

                // Spawn a task that deals with this request.
                task_handler(new_message.content, history, &mut sender).await;
            } // end Message::Text()
            // TODO: Develop a tool for dealing with other types of requests.
            _ => (),
        } // end match
    } // end while
} // end fn socket_handler()

// This function deals with the request for ChatGPT that require
// a text answer.
async fn task_handler(
    new_message: String,
    history: Vec<ChatMessage>,
    sender: &mut SplitSink<WebSocket, Message>,
) {
    let mut stream = match get_answer_stream(new_message.clone(), history).await {
        Ok(stream) => stream,
        Err(_) => {
            // TODO: Inform the client that an error occurred.
            return;
        } // end Err()
    }; // end let stream.

    while let Some(chunk) = stream.next().await {
        println!("Got a chunk from ChatGPT");
        match chunk {
            ResponseChunk::Content {
                delta,
                response_index,
            } => {
                println!("{}", delta);
                // Send a chunk to the client.
                sender
                    .send(Message::Text(delta))
                    .await
                    .expect("Failed to send a message to the client");
            }
            other => (),
        }
    }
} // end fn task_handler()
async fn get_answer_stream(
    message: String,
    mut messages: Vec<ChatMessage>,
) -> Result<impl Stream<Item = ResponseChunk>> {
    dotenv().ok();
    // let (message, chat_id, user_id) = parse_json(json_str).unwrap();
    // let mut messages: Vec<ChatMessage> = serde_json::from_str(json_conversation).unwrap();

    //remove log
    if !messages.is_empty() {
        messages.remove(0);
        let new_message = ChatMessage {
            role: Role::System,
            content: format!(
                "You are ChatGPT, an AI model developed by OpenAI.\
                Answer as concisely as possible. Today is: {0}",
                Local::now().format("%d/%m/%Y %H:%M")
            ),
        };
        messages.insert(0, new_message);
    }
    //add new log

    // Creating a client
    let key = env::var("OAI_TOKEN").unwrap();
    println!("ChatGPT key is imported");
    let client = ChatGPT::new(key)?;
    let mut conversation = client.new_conversation();
    if !messages.is_empty() {
        conversation.history = messages;
    }

    // Acquiring a streamed response
    // Note, that the `futures_util` crate is required for most
    // stream related utility methods
    let stream = conversation.send_message_streaming(message).await?;
    println!("Stream with ChatGPT is established successfully");

    // Iterating over a stream and collecting the results into a vector
    // let mut output: Vec<ResponseChunk> = Vec::new();
    // while let Some(chunk) = stream.next().await {
    //     match chunk {
    //         ResponseChunk::Content {
    //             delta,
    //             response_index,
    //         } => {
    //             // Printing part of response without the newline
    //             print!("{delta}");
    //             // Manually flushing the standard output, as `print` macro does not do that
    //             stdout().lock().flush().unwrap();
    //             output.push(ResponseChunk::Content {
    //                 delta,
    //                 response_index,
    //             });
    //         }
    //         other => output.push(other),
    //     }
    // }
    // // Parsing ChatMessage from the response chunks and saving it to the conversation history
    // output
    Ok(stream)
}
