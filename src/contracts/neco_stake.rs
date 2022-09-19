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
    NecoStakeContract,
    "./src/abis/stake_neco.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

#[derive(Debug)]
pub struct NecoStake {
    contract: NecoStakeContract<Provider<Http>>,
}

impl NecoStake {
    pub fn new(network: NetworkType) -> NecoStake {
        let client = ProviderManager::instance()
            .get_provider(NetworkType::BSCTestNetwork)
            .expect("get provider failed");
        let address = AddressManager::default()
            .get_contract_address(&ContractType::StakeNecoForFee, &network)
            .expect("get contract address failed");
        let contract = NecoStakeContract::new(address, client.clone());
        NecoStake { contract }
    }
}

impl NecoStake {
    pub async fn get_neco_staked_amount(&self, account: &str) -> Result<U256, Error> {
        let address = account.parse::<Address>()?;
        Ok(self.contract.get_staked_neco_amount(address).call().await?)
    }

    pub async fn get_neco_staked_time(&self, account: &str) -> Result<U256, Error> {
        let address = account.parse::<Address>()?;
        Ok(self.contract.get_staked_time_period(address).call().await?)
    }
}
