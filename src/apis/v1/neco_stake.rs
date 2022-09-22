use axum::{extract::Path, http::StatusCode, Json};
use ethers::types::U256;

use crate::{
    apis::response::response_model::NecoResponse, common::defines::NetworkType,
    models::NECOStakedInfo, services::neco_stake::NecoStakeService,
};

// get neco staked info by public address
pub async fn get_neco_staked_info(
    Path((network, public_address)): Path<(u8, String)>,
) -> Json<NecoResponse<NECOStakedInfo>> {
    println!("public_address: {}", public_address);
    println!("network: {:?}", network);
    let network = match network {
        0 => NetworkType::BSCMainNetwork,
        1 => NetworkType::BSCTestNetwork,
        _ => return NecoResponse::err(StatusCode::BAD_REQUEST, "network type error"),
    };

    let staked_amount = NecoStakeService::new(network)
        .get_neco_staked_amount(&public_address)
        .await
        .unwrap_or_else(|_| U256::from(0));
    let staked_time = NecoStakeService::new(network)
        .get_neco_staked_time(&public_address)
        .await
        .unwrap_or_else(|_| U256::from(0));

    NecoResponse::ok(NECOStakedInfo {
        public_address,
        staked_amount: staked_amount.to_string(),
        staked_time: staked_time.to_string(),
    })
}
