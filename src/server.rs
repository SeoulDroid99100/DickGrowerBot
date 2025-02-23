use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn run_webserver() {
    let app = Router::new().route("/", get(|| async { "Bot is alive!" }));

    let addr = SocketAddr::from(([0, 0, 0, 0], 7860)); // Hugging Face Spaces default port
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("🚀 Web server running on http://{}", addr);

    axum::Server::from_tcp(listener).unwrap().serve(app.into_make_service()).await.unwrap();
}￼Enter
