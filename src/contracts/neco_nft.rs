use std::{
    any::type_name,
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
    sync::Arc,
};

use crate::{
    common::{
        address_manager::AddressManager,
        defines::{ContractType, Error, GameClient, NetworkType, NECO_FISHING_NFT_IDS},
        provider_manager::ProviderManager,
    },
    models::{NecoNFTMetadata, NecoNFTOwnership, OwnershipItem},
};
use ethers::{
    prelude::{abigen, Lazy},
    providers::{call_raw::balance, Http, Provider},
    types::{transaction::request, Address, U256},
};
use tokio::sync::{mpsc, Mutex};

abigen!(
    NecoNFTContract,
    "./src/abis/neco_nft.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

static NFT_URL_CACHES: Lazy<Mutex<HashMap<U256, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
static NFT_METADATA_CACHES: Lazy<Mutex<HashMap<U256, NecoNFTMetadata>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Clone)]
pub struct NecoNFT {
    contract: NecoNFTContract<Provider<Http>>,
}

impl NecoNFT {
    pub fn new(network: NetworkType) -> NecoNFT {
        let client = ProviderManager::instance()
            .get_provider(network)
            .expect("get provider failed");
        let address = AddressManager::default()
            .get_contract_address(&ContractType::NecoNFT, &network)
            .expect("get contract address failed");
        let contract = NecoNFTContract::new(address, client.clone());
        NecoNFT { contract }
    }
}

impl NecoNFT {
    pub async fn get_nft_ownership(
        &self,
        public_address: &str,
        game: &GameClient,
        network: &NetworkType,
    ) -> Result<NecoNFTOwnership, Error> {
        let contract_address = AddressManager::default()
            .get_contract_address(&ContractType::NecoNFT, network)?
            .to_string();
        let ownership_items = self.get_ownership_items(public_address, game).await?;
        Ok(NecoNFTOwnership {
            public_address: public_address.to_owned(),
            network: network.to_owned(),
            contract_address: contract_address,
            ownerships: ownership_items,
        })
    }

    pub async fn get_ownership_items(
        &self,
        public_address: &str,
        game: &GameClient,
    ) -> Result<Vec<OwnershipItem>, Error> {
        let neco_nft = Arc::new(self.clone());
        let address = public_address.parse::<Address>().unwrap();
        let (tx, mut rx) = mpsc::channel(4096);

        let mut ids = NECO_FISHING_NFT_IDS;
        match game {
            GameClient::NecoFishing => ids = NECO_FISHING_NFT_IDS,
        }

        for nft_id in ids {
            let neco_nft_copy = neco_nft.clone();
            let tx_copy = tx.clone();

            tokio::spawn(async move {
                let balance = (*neco_nft_copy)
                    .borrow()
                    .contract
                    .balance_of(
                        address.clone(),
                        U256::from_dec_str(&nft_id.to_string()).unwrap(),
                    )
                    .call()
                    .await
                    .unwrap_or_default();

                if balance.as_u64() == 0 {
                    tx_copy
                        .send(OwnershipItem {
                            nft_id: nft_id.to_string(),
                            amount: 0,
                            nft_metadata: NecoNFTMetadata::default(),
                        })
                        .await
                        .unwrap();
                } else {
                    let metadata = (*neco_nft_copy)
                        .borrow()
                        .get_metadata_by_id(&U256::from_dec_str(&nft_id.to_string()).unwrap())
                        .await
                        .unwrap_or_else(|_| NecoNFTMetadata::default());
                    tx_copy
                        .send(OwnershipItem {
                            nft_id: nft_id.to_string(),
                            amount: balance.as_u64(),
                            nft_metadata: metadata,
                        })
                        .await
                        .unwrap();
                }
            });
        }

        let mut ownership_items: Vec<OwnershipItem> = vec![];
        for _ in 0..ids.len() {
            if let Some(ownership_item) = rx.recv().await {
                if ownership_item.amount != 0 {
                    ownership_items.push(ownership_item);
                }
            }
        }
        ownership_items.sort_by(|a, b| a.nft_id.cmp(&b.nft_id));
        Ok(ownership_items)
    }

    pub async fn get_metadata_by_id(&self, nft_id: &U256) -> Result<NecoNFTMetadata, Error> {
        let map = NFT_METADATA_CACHES.lock().await;
        let result = map.get(&nft_id).cloned();
        std::mem::drop(map);

        match result {
            Some(metadata) => Ok(metadata),
            None => {
                // 1. get nft url
                let url = self.get_nft_url(nft_id).await?;
                let metadata_url = format!("https://mygateway.mypinata.cloud/{}", url);
                let requester = reqwest::Client::new();
                let result = requester.get(metadata_url).send().await?.text().await?;
                let metadata: NecoNFTMetadata = serde_json::from_str(&result)?;
                NFT_METADATA_CACHES
                    .lock()
                    .await
                    .insert(nft_id.into(), metadata.to_owned());
                Ok(metadata)
            }
        }
    }

    pub async fn get_nft_url(&self, nft_id: &U256) -> Result<String, Error> {
        let map = NFT_URL_CACHES.lock().await;
        let result = map.get(&nft_id).cloned();
        std::mem::drop(map);

        match result {
            Some(url) => Ok(url),
            None => {
                let url = self.contract.uri(nft_id.into()).call().await?;
                NFT_URL_CACHES
                    .lock()
                    .await
                    .insert(nft_id.into(), url.clone());
                Ok(url)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{common::defines::ContractType, contracts::neco_nft::NecoNFT};
    use ethers::types::U256;

    #[test]
    fn test_get_nft_metadata() {
        let neco_nft = NecoNFT::new(crate::common::defines::NetworkType::BSCTestNetwork);
        let metadata = tokio_test::block_on(
            neco_nft.get_metadata_by_id(&U256::from_dec_str("10001").unwrap()),
        );
        println!("{:?}", metadata);
    }
}
