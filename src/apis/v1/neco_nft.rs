use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use ethers::types::U256;
use std::borrow::Borrow;

use crate::{
    apis::{request::request_model::GetNFTOwnershipRequest, response::response_model::Response},
    common::defines::{GameClient, NetworkType},
    models::{NecoNFTMetadata, NecoNFTOwnership},
    services::neco_nft::NecoNFTService,
};

// get nft metadata by nft id
pub async fn get_nft_metadata(
    Path((network, nft_id)): Path<(u8, String)>,
) -> Json<Response<NecoNFTMetadata>> {
    let network = match network {
        0 => NetworkType::BSCMainNetwork,
        1 => NetworkType::BSCTestNetwork,
        _ => {
            return Response::err(StatusCode::BAD_REQUEST, "network type error");
        }
    };
    let neco_nft = NecoNFTService::new(network);
    let ownership = neco_nft
        .get_metadata_by_id(&U256::from_dec_str((&nft_id).borrow()).unwrap())
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
        (status = 200, description = "Get ERC20 token balance successfully", body = ERC1155OwnershipResponse),
        (status = 400, description = "Bad request", body = ErrorReponse),
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
