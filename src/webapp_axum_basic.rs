use axum::{routing::get, Router};

pub async fn start_server() -> std::io::Result<()> {
    let app = Router::new().route("/", get(|| async { "Hello, World! Axum" }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await
}
