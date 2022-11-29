use crate::{
    common::{
        address::get_contract_address,
        defines::{
            Error, GameClient, NetworkType, SupportedContractType, NAMILAND_GAME_ITEM_NFT_IDS,
        },
        provider::ProviderManager,
    },
    models::{NamiLandERC1155NFTMetadata, NamiLandNFTOwnership, OwnershipItem},
};
use ethers::{
    prelude::{abigen, Lazy},
    providers::{Http, Provider},
    types::{Address, U256},
};
use std::{borrow::Borrow, collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, Mutex};

abigen!(
    NamiLandERC1155Contract,
    "./src/abi/namiland-game-item.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

static NFT_URL_CACHES: Lazy<Mutex<HashMap<U256, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
static NFT_METADATA_CACHES: Lazy<Mutex<HashMap<U256, NamiLandERC1155NFTMetadata>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Clone)]
pub struct NamiLandERC1155Service {
    pub contract: NamiLandERC1155Contract<Provider<Http>>,
}

impl NamiLandERC1155Service {
    pub fn new(network: NetworkType) -> Result<NamiLandERC1155Service, Error> {
        let client = match ProviderManager::instance().get_provider(network) {
            Some(client) => client,
            None => return Err("get provider failed".into()),
        };
        let address = get_contract_address(SupportedContractType::NamiLandGameItemNFT, network)?;
        let contract = NamiLandERC1155Contract::new(address, client.clone());
        Ok(NamiLandERC1155Service { contract })
    }
}

impl NamiLandERC1155Service {
    pub async fn get_nft_ownership(
        &self,
        public_address: Address,
        game_client: GameClient,
        network: NetworkType,
    ) -> Result<NamiLandNFTOwnership, Error> {
        let ownership_items = self
            .get_ownership_items(public_address, game_client)
            .await?;

        let contract_address =
            get_contract_address(SupportedContractType::NamiLandGameItemNFT, network)?.to_string();

        Ok(NamiLandNFTOwnership {
            public_address: format!("{:?}", public_address),
            network: network.to_owned(),
            contract_address,
            ownerships: ownership_items,
        })
    }

    // Get ownership items
    pub async fn get_ownership_items(
        &self,
        public_address: Address,
        game_client: GameClient,
    ) -> Result<Vec<OwnershipItem>, Error> {
        let neco_nft = Arc::new(self.clone());
        let (tx, mut rx) = mpsc::channel(4096);

        let nft_ids: Vec<i32> = match game_client {
            GameClient::NamiLand => {
                let mut ids: Vec<i32> = vec![];
                NAMILAND_GAME_ITEM_NFT_IDS
                    .iter()
                    .for_each(|id| ids.push(*id));
                ids
            }
        };

        nft_ids.clone().into_iter().for_each(|id: i32| {
            let neco_nft_copy = neco_nft.clone();
            let tx_copy = tx.clone();

            tokio::spawn(async move {
                let balance = (*neco_nft_copy)
                    .borrow()
                    .contract
                    .balance_of(public_address.clone(), U256::from(id))
                    .call()
                    .await
                    .unwrap_or_default();

                let metadata = match balance.as_u64() {
                    0 => NamiLandERC1155NFTMetadata::default(),
                    _ => (*neco_nft_copy)
                        .borrow()
                        .get_metadata_by_nft_id(&U256::from(id))
                        .await
                        .unwrap_or_else(|_| NamiLandERC1155NFTMetadata::default()),
                };

                tx_copy
                    .send(OwnershipItem {
                        nft_id: id.to_string(),
                        amount: balance.as_u64(),
                        nft_metadata: metadata,
                    })
                    .await
                    .expect("TODO: panic message");
            });
        });

        let mut ownership_items: Vec<OwnershipItem> = vec![];
        for _ in 0..nft_ids.len() {
            if let Some(ownership_item) = rx.recv().await {
                if ownership_item.amount != 0 {
                    ownership_items.push(ownership_item);
                }
            }
        }
        ownership_items.sort_by(|a, b| a.nft_id.cmp(&b.nft_id));
        Ok(ownership_items)
    }

    // get nft metadata by nft id
    pub async fn get_metadata_by_nft_id(
        &self,
        nft_id: &U256,
    ) -> Result<NamiLandERC1155NFTMetadata, Error> {
        let map = NFT_METADATA_CACHES.lock().await;
        let result = map.get(&nft_id).cloned();
        drop(map);

        match result {
            Some(metadata) => Ok(metadata),
            None => {
                // 1. get nft url
                let url = self.get_nft_url(nft_id).await?;
                let requester = reqwest::Client::new();
                let result = requester.get(url).send().await?.text().await?;
                let metadata: NamiLandERC1155NFTMetadata = serde_json::from_str(&result)?;
                NFT_METADATA_CACHES
                    .lock()
                    .await
                    .insert(nft_id.into(), metadata.to_owned());
                Ok(metadata)
            }
        }
    }

    // get nft uri.
    pub async fn get_nft_url(&self, nft_id: &U256) -> Result<String, Error> {
        let map = NFT_URL_CACHES.lock().await;
        let result = map.get(&nft_id).cloned();
        drop(map);

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
    use ethers::types::U256;

    use crate::services::namiland_erc1155::NamiLandERC1155Service;

    #[test]
    fn test_get_nft_metadata() {
        let neco_nft =
            NamiLandERC1155Service::new(crate::common::defines::NetworkType::BSCTestNetwork);
        let metadata = tokio_test::block_on(
            neco_nft
                .unwrap()
                .get_metadata_by_nft_id(&U256::from_dec_str("10001").unwrap()),
        );
        println!("{:?}", metadata);
    }
}
