use std::sync::Arc;

use crate::common::{
    address_manager::AddressManager,
    defines::{Error, NetworkType, SupportedContract},
    provider_manager::ProviderManager,
};
use ethers::{
    prelude::abigen,
    providers::{Http, Provider},
};

abigen!(
    NECOContract,
    "./src/abis/erc20.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

#[derive(Debug)]
pub struct NECO {
    contract: NECOContract<Provider<Http>>,
}

impl NECO {
    pub fn new(network: NetworkType) -> NECO {
        let client = Arc::new(
            ProviderManager::instance()
                .get_provider(NetworkType::BSCTestNetwork)
                .expect("get provider failed"),
        );
        let address = AddressManager::default()
            .get_contract_address(SupportedContract::NECOTokenContract, network)
            .expect("get contract address failed");
        let contract = NECOContract::new(address, client.clone());
        NECO { contract }
    }
}

impl NECO {
    pub async fn get_symbol(&self) -> Result<String, Error> {
        Ok(self.contract.symbol().call().await?)
    }
}
