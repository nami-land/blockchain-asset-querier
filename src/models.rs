use serde::{Deserialize, Serialize};

use crate::common::defines::NetworkType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NecoNFTTrait {
    pub trait_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NecoNFTMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(alias = "nftType1")]
    pub nft_type1: String,
    #[serde(alias = "nftType2")]
    pub nft_type2: String,
    pub animation_url: String,
    pub image: String,
    pub external_url: String,
    pub attributes: Vec<NecoNFTTrait>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OwnershipItem {
    pub nft_id: String,
    pub amount: u64,
    pub nft_metadata: NecoNFTMetadata,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NecoNFTOwnership {
    pub public_address: String,
    pub network: NetworkType,
    pub contract_address: String,
    pub ownerships: Vec<OwnershipItem>,
}
