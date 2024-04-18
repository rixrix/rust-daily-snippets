// main.rs
mod webapp_actix_basic;

use webapp_actix_basic::start_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    start_server().await
}
