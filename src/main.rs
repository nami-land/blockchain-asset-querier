mod apis;
mod common;
mod models;
mod router;
mod services;

use crate::common::defines::{Error, BSC_MAIN_NETWORK_RPC, BSC_TEST_NETWORK_RPC};
use crate::models::{Address, AddressConfig};
use common::defines::NetworkType;
use common::provider::ProviderManager;
use ethers::providers::{Http, Provider};
use log::{info, LevelFilter};
use once_cell::sync::OnceCell;
use std::fs;
use std::net::SocketAddr;

static ADDRESS_MANAGER: OnceCell<AddressConfig> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(LevelFilter::Info)
        .init();

    // let env_args: HashMap<String, String> = env::vars().collect();
    // let env = env_args.get("env");
    // match env {
    //     Some(env) => println!("env: {}", env),
    //     None => println!("env is none"),
    // };

    let address_content = fs::read_to_string("address.toml")?;
    let address: AddressConfig = toml::from_str(&address_content)?;
    println!("address: {:?}", address);
    ADDRESS_MANAGER.get_or_init(|| address);

    let bsc_main_client =
        Provider::<Http>::try_from(BSC_MAIN_NETWORK_RPC).expect("get bsc mainnet provider failed.");
    let bsc_test_client =
        Provider::<Http>::try_from(BSC_TEST_NETWORK_RPC).expect("get bsc testnet provider failed.");

    ProviderManager::instance().set_provider(NetworkType::BSCMainNetwork, bsc_main_client);
    ProviderManager::instance().set_provider(NetworkType::BSCTestNetwork, bsc_test_client);

    let app = router::new_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("web server is listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
