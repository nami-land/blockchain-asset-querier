use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetERC20BalanceRequest {
    pub chain_id: u8,
    pub contract_type: String,
    pub public_address: String,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetNFTOwnershipRequest {
    pub chain_id: u8,
    pub game_client: u8,
    pub public_address: String,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetNECOStakedInfoRequest {
    pub chain_id: u8,
    pub public_address: String,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetERC1155NFTMetadataRequest {
    pub chain_id: u8,
    pub nft_id: String,
}
