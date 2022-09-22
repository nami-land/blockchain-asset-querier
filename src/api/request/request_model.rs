use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetErc20BalanceRequest {
    pub network: u8,
    pub contract_type: String,
    pub public_address: String,
}

#[derive(Debug, Deserialize)]
pub struct GetNftOwnershipRequest {
    pub network: u8,
    pub game_client: u8,
    pub public_address: String,
}
