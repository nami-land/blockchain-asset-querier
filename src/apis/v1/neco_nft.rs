use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use ethers::types::U256;
use std::borrow::Borrow;

use crate::{
    apis::{
        request::request_model::{GetNFTMetadataRequest, GetNFTOwnershipRequest},
        response::response_model::Response,
    },
    common::defines::{GameClient, NetworkType},
    models::{NecoNFTMetadata, NecoNFTOwnership},
    services::neco_nft::NecoNFTService,
};

// get nft metadata by nft id
#[utoipa::path(
    get,
    path = "/v1/nft/metadata/{network}/{nft_id}",
    tag = "ERC1155",
    params(
        GetNFTMetadataRequest
    ),
    responses(
        (status = 200, description = "Get NFT metadata successfully", body = ERC1155OwnershipResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
    )
)]
pub async fn get_nft_metadata(
    Path(param): Path<GetNFTMetadataRequest>,
) -> Json<Response<NecoNFTMetadata>> {
    let network = match param.network {
        0 => NetworkType::BSCMainNetwork,
        1 => NetworkType::BSCTestNetwork,
        _ => {
            return Response::err(StatusCode::BAD_REQUEST, "network type error");
        }
    };
    let neco_nft = NecoNFTService::new(network);
    let ownership = neco_nft
        .get_metadata_by_id(&U256::from_dec_str((&param.nft_id).borrow()).unwrap())
        .await;

    match ownership {
        Ok(metadata) => Response::ok(metadata),
        Err(e) => Response::err(StatusCode::BAD_REQUEST, &e.to_string()),
    }
}

// get nft ownership by public address
#[utoipa::path(
    get,
    path = "/v1/nft/ownership",
    tag = "ERC1155",
    params(
        GetNFTOwnershipRequest
    ),
    responses(
        (status = 200, description = "Get NFT ownership successfully", body = ERC1155OwnershipResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
    )
)]
pub async fn get_nft_ownership(
    Query(request): Query<GetNFTOwnershipRequest>,
) -> Json<Response<NecoNFTOwnership>> {
    let game_client = match request.game_client {
        0 => GameClient::NecoFishing,
        _ => return Response::err(StatusCode::BAD_REQUEST, "game client type error"),
    };
    let network = match request.network {
        0 => NetworkType::BSCMainNetwork,
        1 => NetworkType::BSCTestNetwork,
        _ => return Response::err(StatusCode::BAD_REQUEST, "network type error"),
    };

    let neco_nft = NecoNFTService::new(network);
    let ownership = neco_nft
        .get_nft_ownership(&request.public_address, &game_client, &network)
        .await;

    match ownership {
        Ok(ownership) => Response::ok(ownership),
        Err(err) => Response::err(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string()),
    }
}
