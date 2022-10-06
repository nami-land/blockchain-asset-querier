use crate::common::defines::NetworkType;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct ERC20Token {
    pub symbol: String,
    pub decimal: u8,
    pub amount: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct NECOStakedInfo {
    pub public_address: String,
    pub staked_amount: String,
    pub staked_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NecoNFTTrait {
    pub trait_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct OwnershipItem {
    pub nft_id: String,
    pub amount: u64,
    pub nft_metadata: NecoNFTMetadata,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct NecoNFTOwnership {
    pub public_address: String,
    pub network: NetworkType,
    pub contract_address: String,
    pub ownerships: Vec<OwnershipItem>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct EmptyData {}

#[derive(Debug, Deserialize)]
pub struct AddressConfig {
    pub bsc_test: Address,
    pub bsc_main: Address,
}

#[derive(Debug, Deserialize)]
pub struct Address {
    pub neco: String,
    pub nfish: String,
    pub busd: String,
    pub neco_nft: String,
    pub stake_neco_for_fee: String,
}
