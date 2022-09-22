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
    NecoStakeContract,
    "./src/abis/stake_neco.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

#[derive(Debug)]
pub struct NecoStakeService {
    contract: NecoStakeContract<Provider<Http>>,
}

impl NecoStakeService {
    pub fn new(network: NetworkType) -> NecoStakeService {
        let client = ProviderManager::instance()
            .get_provider(network)
            .expect("get provider failed");
        let address = get_contract_address(&ContractType::StakeNecoForFee, &network)
            .expect("get contract address failed");
        let contract = NecoStakeContract::new(address, client.clone());
        NecoStakeService { contract }
    }
}

impl NecoStakeService {
    pub async fn get_neco_staked_amount(&self, account: &str) -> Result<U256, Error> {
        let address = account.parse::<Address>()?;
        Ok(self.contract.get_staked_neco_amount(address).call().await?)
    }

    pub async fn get_neco_staked_time(&self, account: &str) -> Result<U256, Error> {
        let address = account.parse::<Address>()?;
        Ok(self.contract.get_staked_time_period(address).call().await?)
    }
}
