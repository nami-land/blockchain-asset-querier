use std::string;

use crate::common::{
    address::get_contract_address,
    defines::{ContractType, Error, NetworkType},
    provider::ProviderManager,
};
use ethers::{
    prelude::abigen,
    providers::{Http, Provider},
    types::{Address, U256},
};

abigen!(
    ERC20Contract,
    "./src/abis/erc20.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

#[derive(Debug)]
pub struct ERC20 {
    contract: ERC20Contract<Provider<Http>>,
}

impl ERC20 {
    pub fn new(contract_type: ContractType, network: NetworkType) -> ERC20 {
        let client = ProviderManager::instance()
            .get_provider(network)
            .expect("get provider failed");
        let address =
            get_contract_address(&contract_type, &network).expect("get contract address failed");
        let contract = ERC20Contract::new(address, client.clone());
        ERC20 { contract }
    }
}

impl ERC20 {
    pub async fn get_symbol(&self) -> Result<String, Error> {
        Ok(self.contract.symbol().call().await?)
    }

    pub async fn get_decimal(&self) -> Result<u8, Error> {
        Ok(self.contract.decimals().call().await?)
    }

    pub async fn get_balance(&self, account: &str) -> Result<U256, Error> {
        let address = account.parse::<Address>()?;
        Ok(self.contract.balance_of(address).call().await?)
    }
}
