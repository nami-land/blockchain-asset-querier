use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use ethers::types::U256;

use crate::{
    apis::{
        request::request_model::GetNFTOwnershipRequest, response::response_model::NecoResponse,
    },
    common::defines::{GameClient, NetworkType},
    models::{NecoNFTMetadata, NecoNFTOwnership},
    services::neco_nft::NecoNFTService,
};

pub async fn get_nft_metadata(
    Path((network, nft_id)): Path<(u8, String)>,
) -> Json<NecoResponse<NecoNFTMetadata>> {
    let network = match network {
        0 => NetworkType::BSCMainNetwork,
        1 => NetworkType::BSCTestNetwork,
        _ => {
            return NecoResponse::err(StatusCode::BAD_REQUEST, "network type error");
        }
    };
    let neco_nft = NecoNFTService::new(network);
    let ownership = neco_nft
        .get_metadata_by_id(&U256::from_dec_str(&nft_id).unwrap())
        .await;

    match ownership {
        Ok(metadata) => NecoResponse::ok(metadata),
        Err(e) => NecoResponse::err(StatusCode::BAD_REQUEST, &e.to_string()),
    }
}

// get nft ownership by public address
pub async fn get_nft_ownership(
    Query(request): Query<GetNFTOwnershipRequest>,
) -> Json<NecoResponse<NecoNFTOwnership>> {
    println!("{:?}", request);
    let game_client = match request.game_client {
        0 => GameClient::NecoFishing,
        _ => return NecoResponse::err(StatusCode::BAD_REQUEST, "game client type error"),
    };
    let network = match request.network {
        0 => NetworkType::BSCMainNetwork,
        1 => NetworkType::BSCTestNetwork,
        _ => return NecoResponse::err(StatusCode::BAD_REQUEST, "network type error"),
    };

    let neco_nft = NecoNFTService::new(network);
    // let public_address = request.public_address.as_str();
    let ownership = neco_nft
        .get_nft_ownership(&request.public_address, &game_client, &network)
        .await;

    match ownership {
        Ok(ownership) => NecoResponse::ok(ownership),
        Err(err) => NecoResponse::err(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string()),
    }
}
