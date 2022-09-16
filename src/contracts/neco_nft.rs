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
    NecoNFTContract,
    "./src/abis/neco_nft.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

#[derive(Debug)]
pub struct NecoNFT {
    contract: NecoNFTContract<Provider<Http>>,
}

impl NecoNFT {
    pub fn new(contract_type: ContractType, network: NetworkType) -> NecoNFT {
        let client = ProviderManager::instance()
            .get_provider(NetworkType::BSCTestNetwork)
            .expect("get provider failed");
        let address = AddressManager::default()
            .get_contract_address(contract_type, network)
            .expect("get contract address failed");
        let contract = NecoNFTContract::new(address, client.clone());
        NecoNFT { contract }
    }
}

impl NecoNFT {}
