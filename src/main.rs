mod common;

use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::common::defines::{
    Error, SupportedContract, BSC_MAIN_NETWORK_RPC, BSC_TEST_NETWORK_RPC,
};
use axum::{routing::get, Router};
use common::{
    contracts::AddressManager,
    defines::{NetworkType, SupportedERC20Token},
};
use ethers::abi::Abi;
use ethers::contract::Contract;
use ethers::{
    prelude::{abigen, SignerMiddleware},
    providers::{Http, Provider},
};

abigen!(
    ERC20Contract,
    "./src/abis/erc20.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = Router::new().route("/", get(hello_world));

    let bsc_main_client = Provider::<Http>::try_from(BSC_MAIN_NETWORK_RPC).unwrap();
    let bsc_test_client = Provider::<Http>::try_from(BSC_TEST_NETWORK_RPC).unwrap();

    let abi_str = fs::read_to_string("./src/abis/erc20.json").unwrap();
    let abi: Abi = serde_json::from_str(&abi_str).unwrap();

    let address = AddressManager::default()
        .get_contract_address(
            SupportedContract::NECOTokenContract,
            NetworkType::BSCTestNetwork,
        )
        .unwrap();
    let contract = Contract::new(address, abi, bsc_test_client);

    let symbol: String = contract.method::<_, String>("symbol", ())?.call().await?;
    println!("symbol is {}", symbol);

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
