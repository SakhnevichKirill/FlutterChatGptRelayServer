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

use dotenvy::dotenv;
use futures_util::{stream::SplitSink, SinkExt, Stream, StreamExt};
use std::env;
use std::net::SocketAddr;

use crate::models::ClientReq;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{BufRead, BufReader};
use chrono::Local;
lazy_static::lazy_static! {
    static ref HASH_MAP: std::sync::RwLock<HashMap<String, String>> = {
        let mut map = HashMap::new();
        std::sync::RwLock::new(map)
    };
}



pub fn fill_prompts() {

    let file = File::open("promts.csv").expect("Failed to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split(',').collect();
          match HASH_MAP.write() {
        Ok(mut map) => {
            map.insert(parts[0].trim_matches('"').parse().unwrap(),
                       parts[1].trim_matches('"').parse().unwrap());
        },
        Err(e) => panic!("Ошибка доступа к глобальной переменной HASH_MAP: {:?}", e),
    }

    }
}
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
        Err(err) => {
            println!("Error with stream establishing: {}", err);
            // TODO: Inform the client that an error occurred.
            return;
        } // end Err()
    }; // end let stream.

    while let Some(chunk) = stream.next().await {
        println!("Got a chunk from ChatGPT");
        match chunk {
            ResponseChunk::Content {
                delta,
                response_index: _,
            } => {
                println!("{}", delta);
                // Send a chunk to the client.
                sender
                    .send(Message::Text(delta))
                    .await
                    .expect("Failed to send a message to the client");
            } // end ResponseChunk::Content

            ResponseChunk::CloseResponse { response_index: _ } => {
                // Inform a client that message sending is finished.
                sender
                    .send(Message::Close(None))
                    .await
                    .expect("Failed to send a message to the client");
            } // end ResponseChunk::CloseResponse

            other => {
                println!("{:#?}", other);
            } // end other
        } // end match
    } // end while let
} // end fn task_handler()

// This function prompts ChatGPT and returns a stream,
// which it is possible to read data by chunks from.
async fn get_answer_stream(
    message: String,
    mut messages: Vec<ChatMessage>,
) -> Result<impl Stream<Item = ResponseChunk>> {
    dotenv().ok();
    // let (message, chat_id, user_id) = parse_json(json_str).unwrap();
    // let mut messages: Vec<ChatMessage> = serde_json::from_str(json_conversation).unwrap();
    let new_message = ChatMessage {
        role: Role::System,
        content: format!(
            "You are ChatGPT, an AI model developed by OpenAI.\
         Answer as concisely as possible. Today is: {0}", Local::now().format("%d/%m/%Y %H:%M")
        ),
    };
    messages.insert(0, new_message);
    // Make ChatGPT setup.
    //if messages.len() > 0 {
      //   messages.remove(0);
        // messages.insert(0, new_message);
     //} else {
       //  messages.push(new_message);
     //} // end if
    //TODO необходимо пройтись по всей истории найти все роли System и в момент,
    // когда встречаем эту роль отредактировать историю update метод по индексу

    // Creating a client
    let key = env::var("OAI_TOKEN").unwrap();
    println!("ChatGPT key is imported");
    let client = ChatGPT::new(key)?;
    let mut conversation = client.new_conversation();
   // if !messages.is_empty() {
    conversation.history = messages;
   // }

    // Acquiring a streamed response
    // Note, that the `futures_util` crate is required for most
    // stream related utility methods
    let stream = conversation.send_message_streaming(message).await?;
    println!("Stream with ChatGPT is established successfully");

    Ok(stream)
}
