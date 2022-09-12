use std::net::SocketAddr;

use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let app = Router::new()
        .route("/", get(hello_world));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8888));
    println!("web server is listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world() -> &'static str {
    "hello world"
}
