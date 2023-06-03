mod ws_handler;

use axum::{routing::get, Router};

use ws_handler::ws_handler;

// This function creates an application to run on the server.
// Basically, it creates the application router.
pub async fn create_app() -> Router {
    // Return the application (router) for the server.
    Router::new().route("/ws", get(ws_handler))
} // end fn create_app()
