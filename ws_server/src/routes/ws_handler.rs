use std::net::SocketAddr;

use axum::{
    extract::{connect_info::ConnectInfo, ws::WebSocket, WebSocketUpgrade},
    headers::UserAgent,
    response::IntoResponse,
    TypedHeader,
};

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
async fn socket_handler(mut _socket: WebSocket, addr: SocketAddr) {
    println!("A user with IP: {} is connected to a websocket", addr.ip());
} // end fn socket_handler()
