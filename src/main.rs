mod apis;
mod common;
mod models;
mod router;
mod services;

use crate::common::defines::{Error, BSC_MAIN_NETWORK_RPC, BSC_TEST_NETWORK_RPC};
use common::defines::NetworkType;
use common::provider::ProviderManager;
use ethers::providers::{Http, Provider};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let bsc_main_client =
        Provider::<Http>::try_from(BSC_MAIN_NETWORK_RPC).expect("get bsc mainnet provider failed.");
    let bsc_test_client =
        Provider::<Http>::try_from(BSC_TEST_NETWORK_RPC).expect("get bsc testnet provider failed.");

    ProviderManager::instance().set_provider(NetworkType::BSCMainNetwork, bsc_main_client);
    ProviderManager::instance().set_provider(NetworkType::BSCTestNetwork, bsc_test_client);

    let app = router::new_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("web server is listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
