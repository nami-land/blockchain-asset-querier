use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use ethers::types::U256;

use crate::{
    api::{request::request_model::GetNftOwnershipRequest, response::response_model::NecoResult},
    common::defines::{GameClient, NetworkType},
    contracts::neco_nft::NecoNFT,
    models::{NecoNFTMetadata, NecoNFTOwnership},
};

pub async fn get_nft_metadata(
    Path((network, nft_id)): Path<(u8, String)>,
) -> Json<NecoResult<NecoNFTMetadata>> {
    let network = match network {
        0 => NetworkType::BSCMainNetwork,
        1 => NetworkType::BSCTestNetwork,
        _ => {
            return Json(NecoResult {
                status: StatusCode::BAD_REQUEST.as_u16(),
                message: "network type error".to_string(),
                data: None,
            });
        }
    };
    let neco_nft = NecoNFT::new(network);
    let ownership = neco_nft
        .get_metadata_by_id(&U256::from_dec_str(&nft_id).unwrap())
        .await;

    match ownership {
        Ok(metadata) => Json(NecoResult {
            status: StatusCode::OK.as_u16(),
            message: "success".to_string(),
            data: Some(metadata),
        }),
        Err(e) => Json(NecoResult {
            status: StatusCode::BAD_REQUEST.as_u16(),
            message: e.to_string(),
            data: None,
        }),
    }
}

// get nft ownership by public address
pub async fn get_nft_ownership(
    Query(request): Query<GetNftOwnershipRequest>,
) -> Json<NecoResult<NecoNFTOwnership>> {
    println!("{:?}", request);
    let game_client = match request.game_client {
        0 => GameClient::NecoFishing,
        _ => {
            return Json(NecoResult::new(
                StatusCode::BAD_REQUEST,
                "invalid game client".to_string(),
                None,
            ))
        }
    };
    let network = match request.network {
        0 => NetworkType::BSCMainNetwork,
        1 => NetworkType::BSCTestNetwork,
        _ => {
            return Json(NecoResult {
                status: StatusCode::BAD_REQUEST.as_u16(),
                message: "network type error".to_string(),
                data: None,
            });
        }
    };
    let neco_nft = NecoNFT::new(network);
    // let public_address = request.public_address.as_str();
    let ownership = neco_nft
        .get_nft_ownership(&request.public_address, &game_client, &network)
        .await;

    match ownership {
        Ok(ownership) => Json(NecoResult::new(
            StatusCode::OK,
            "success".to_string(),
            Some(ownership),
        )),
        Err(err) => Json(NecoResult::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            err.to_string(),
            None,
        )),
    }
}
