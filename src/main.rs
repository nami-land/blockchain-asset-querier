mod common;
mod contracts;
use crate::common::defines::{Error, BSC_MAIN_NETWORK_RPC, BSC_TEST_NETWORK_RPC};
use crate::contracts::neco::NECO;
use axum::{routing::get, Router};
use common::defines::NetworkType;
use common::provider_manager::ProviderManager;
use ethers::providers::{Http, Provider};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = Router::new().route("/", get(hello_world));

    let bsc_main_client = Provider::<Http>::try_from(BSC_MAIN_NETWORK_RPC).unwrap();
    let bsc_test_client = Provider::<Http>::try_from(BSC_TEST_NETWORK_RPC).unwrap();

    ProviderManager::instance().set_provider(NetworkType::BSCMainNetwork, bsc_main_client);
    ProviderManager::instance().set_provider(NetworkType::BSCTestNetwork, bsc_test_client);

    let neco = NECO::new(NetworkType::BSCTestNetwork);
    println!("{}", neco.get_symbol().await?);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8888));
    println!("web server is listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn hello_world() -> &'static str {
    "hello world"
}
