use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::common::defines::NetworkType;

#[derive(Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct ERC20Token {
    pub symbol: String,
    pub decimal: u8,
    pub amount: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NamiXStakedInfo {
    pub public_address: String,
    pub staked_amount: String,
    pub staked_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all(deserialize = "snake_case", serialize = "camelCase"))]
pub struct NFTTrait {
    pub trait_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
#[serde(rename_all(deserialize = "snake_case", serialize = "camelCase"))]
pub struct NamiLandERC1155NFTMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub nft_type1: String,
    pub nft_type2: String,
    pub image: String,
    pub external_url: String,
    pub attributes: Vec<NFTTrait>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OwnershipItem {
    pub nft_id: String,
    pub amount: u64,
    pub nft_metadata: NamiLandERC1155NFTMetadata,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NamiLandNFTOwnership {
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
