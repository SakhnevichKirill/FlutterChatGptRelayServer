use ws_server::run;

#[tokio::main]
async fn main() {
    // Run the server.
    run().await;
} // end fn main()
