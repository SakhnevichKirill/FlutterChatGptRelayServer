pub mod models;
mod routes;

use std::net::SocketAddr;

use routes::create_app;

// Global variables.

// Url address of the server.
static SERVER_URL: &str = "0.0.0.0:8080";

// This function runs the entire server.
pub async fn run() {
    // Get the application to run on the server.
    // This is the main router.
    let app = create_app().await;

    // Run the axum server.
    axum::Server::bind(&SERVER_URL.parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
} // end fn run()
