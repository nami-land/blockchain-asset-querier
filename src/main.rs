mod common;

use std::net::SocketAddr;

use crate::common::defines::{BSC_MAIN_NETWORK_RPC, BSC_TEST_NETWORK_RPC};
use axum::{routing::get, Router};
use ethers::providers::{Http, Provider};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));

    let bsc_main_client = Provider::<Http>::try_from(BSC_MAIN_NETWORK_RPC).unwrap();
    let bsc_test_client = Provider::<Http>::try_from(BSC_TEST_NETWORK_RPC).unwrap();

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
