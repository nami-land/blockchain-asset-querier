use std::{borrow::BorrowMut, collections::HashMap, sync::Mutex};

use crate::{
    common::{
        address_manager::AddressManager,
        defines::{ContractType, Error, GameClient, NetworkType},
        provider_manager::ProviderManager,
    },
    models::{NecoNFTMetadata, NecoNFTOwnership, OwnershipItem},
};
use ethers::{
    prelude::{abigen, Lazy},
    providers::{Http, Provider},
    types::U256,
};

abigen!(
    NecoNFTContract,
    "./src/abis/neco_nft.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

static NFT_URL_CACHES: Lazy<Mutex<HashMap<U256, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
static NFT_METADATA_CACHES: Lazy<Mutex<HashMap<U256, NecoNFTMetadata>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

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
            .get_contract_address(&contract_type, &network)
            .expect("get contract address failed");
        let contract = NecoNFTContract::new(address, client.clone());
        NecoNFT { contract }
    }
}

impl NecoNFT {
    pub async fn get_nft_ownership(
        &self,
        public_address: &str,
        game_type: &GameClient,
        network: &NetworkType,
    ) -> Result<NecoNFTOwnership, Error> {
        let contract_address = AddressManager::default()
            .get_contract_address(&ContractType::NecoNFT, network)?
            .to_string();
        Ok(NecoNFTOwnership {
            public_address: public_address.to_owned(),
            network: network.to_owned(),
            contract_address: String::from(""),
            ownerships: vec![],
        })
    }

    async fn get_ownership_items(
        &self,
        public_address: &str,
        game: &GameClient,
    ) -> Result<Vec<OwnershipItem>, Error> {
        Ok(vec![])
    }

    async fn get_metadata_by_id(&self, nft_id: &U256) -> Result<NecoNFTMetadata, Error> {
        let map = NFT_METADATA_CACHES.lock().unwrap();
        let result = map.get(nft_id);
        match result {
            Some(metadata) => Ok((*metadata).clone()),
            None => Ok(NecoNFTMetadata::default()),
        }
    }

    async fn get_nft_url(&self, nft_id: U256) -> Result<String, Error> {
        let map = NFT_URL_CACHES.lock().unwrap();
        let result = map.get(&nft_id);
        match result {
            Some(url) => Ok((*url).clone()),
            None => {
                let url = self.contract.uri(nft_id).call().await?;
                NFT_URL_CACHES.lock().unwrap().insert(nft_id, url.clone());
                Ok(url)
            }
        }
    }
}
