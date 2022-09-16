use std::sync::Arc;

use crate::common::{
    address_manager::AddressManager,
    defines::{ContractType, Error, NetworkType},
    provider_manager::ProviderManager,
};
use ethers::{
    prelude::abigen,
    providers::{Http, Provider},
    types::{Address, U256},
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
        let client = ProviderManager::instance()
            .get_provider(NetworkType::BSCTestNetwork)
            .expect("get provider failed");
        let address = AddressManager::default()
            .get_contract_address(ContractType::NECOTokenContract, network)
            .expect("get contract address failed");
        let contract = NECOContract::new(address, client.clone());
        NECO { contract }
    }
}

impl NECO {
    pub async fn get_symbol(&self) -> Result<String, Error> {
        Ok(self.contract.symbol().call().await?)
    }

    pub async fn get_balance(&self, account: &str) -> Result<U256, Error> {
        let address = account.parse::<Address>()?;
        Ok(self.contract.balance_of(address).call().await?)
    }
}
