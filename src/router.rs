use axum::{routing::get, Router};

use crate::api::v1;

pub fn new_router() -> Router {
    Router::new()
        .route("/ping", get(ping))
        .route(
            "/v1/neco-staked-info/:network/:public_address",
            get(v1::neco_stake::get_neco_staked_info),
        )
        .route("/v1/nft/ownership", get(v1::neco_nft::get_nft_ownership))
        .route(
            "/v1/nft/metadata/:network/:nft_id",
            get(v1::neco_nft::get_nft_metadata),
        )
        .route("/v1/erc20/balance", get(v1::erc20::get_erc20_balance))
}

async fn ping() -> &'static str {
    "pong"
}
