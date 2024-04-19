// main.rs
// mod webapp_actix_basic;
// use webapp_actix_basic::start_server;

mod webapp_axum_basic;
use webapp_axum_basic::start_server;

// #[actix_web::main] // actix
#[tokio::main] // axum
async fn main() -> std::io::Result<()> {
    start_server().await
}
