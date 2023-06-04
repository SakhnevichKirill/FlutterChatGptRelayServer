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
use futures_util::{stream::SplitSink, SinkExt, StreamExt};
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

                // Deserialize the request.
                let ClientReq {
                    new_message,
                    history,
                } = serde_json::from_str(&msg).expect("Failed to convert the passed json");

                // Spawn a task that deals with this request.
                task_handler("".to_string(), &mut sender).await;
            } // end Message::Text()
            // TODO: Develop a tool for dealing with other types of requests.
            _ => (),
        } // end match
    } // end while
} // end fn socket_handler()

// This function deals with the request for ChatGPT that require
// a text answer.
async fn task_handler(msg: String, sender: &mut SplitSink<WebSocket, Message>) {
    sender
        .send(Message::Text(msg))
        .await
        .expect("Failed to send a message to the client");
} // end fn task_handler()
