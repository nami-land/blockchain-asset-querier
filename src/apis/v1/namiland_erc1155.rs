use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use ethers::types::U256;
use std::borrow::Borrow;

use crate::{
    apis::{
        request::request_model::{GetERC1155NFTMetadataRequest, GetNFTOwnershipRequest},
        response::response_model::Response,
    },
    common::defines::{GameClient, NetworkType},
    models::{NamiLandERC1155NFTMetadata, NamiLandNFTOwnership},
    services::namiland_erc1155::NamiLandERC1155Service,
};

// get nft metadata by nft id
#[utoipa::path(
    get,
    path = "/v1/namiland-game-item-nft/metadata/{chain_id}/{nft_id}",
    tag = "ERC1155",
    params(
        GetERC1155NFTMetadataRequest
    ),
    responses(
        (status = 200, description = "Get NFT metadata successfully", body = ERC1155OwnershipResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
    )
)]
pub async fn get_nft_metadata(
    Path(param): Path<GetERC1155NFTMetadataRequest>,
) -> Json<Response<NamiLandERC1155NFTMetadata>> {
    let network = match param.chain_id {
        1 => NetworkType::EthereumMainnet,
        5 => NetworkType::GoerliTestnet,
        _ => {
            return Response::err(StatusCode::BAD_REQUEST, "chain id is not supported");
        }
    };
    let nft_id = match U256::from_dec_str(param.nft_id.borrow()) {
        Ok(nft_id) => nft_id,
        Err(_) => {
            return Response::err(StatusCode::BAD_REQUEST, "nft id is invalid");
        }
    };

    let erc1155_service = match NamiLandERC1155Service::new(network) {
        Ok(erc1155_service) => erc1155_service,
        Err(e) => {
            return Response::err(StatusCode::BAD_REQUEST, e.to_string().as_str());
        }
    };
    let ownership = erc1155_service.get_metadata_by_nft_id(&nft_id).await;
    match ownership {
        Ok(metadata) => Response::ok(metadata),
        Err(e) => Response::err(StatusCode::BAD_REQUEST, &e.to_string()),
    }
}

// get nft ownership by public address
#[utoipa::path(
    get,
    path = "/v1/namiland-game-item-nft/ownership",
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
) -> Json<Response<NamiLandNFTOwnership>> {
    let game_client = match request.game_client {
        0 => GameClient::NamiLand,
        _ => return Response::err(StatusCode::BAD_REQUEST, "game client type error"),
    };
    let network = match request.chain_id {
        1 => NetworkType::EthereumMainnet,
        5 => NetworkType::GoerliTestnet,
        _ => {
            return Response::err(StatusCode::BAD_REQUEST, "chain id is not supported");
        }
    };
    let pubic_address = match request.public_address.parse::<ethers::types::Address>() {
        Ok(address) => address,
        Err(_) => {
            return Response::err(StatusCode::BAD_REQUEST, "public address is invalid");
        }
    };

    let erc1155_service = match NamiLandERC1155Service::new(network) {
        Ok(erc1155_service) => erc1155_service,
        Err(e) => {
            return Response::err(StatusCode::BAD_REQUEST, e.to_string().as_str());
        }
    };
    let ownership = erc1155_service
        .get_nft_ownership(pubic_address, game_client, network)
        .await;

    match ownership {
        Ok(ownership) => Response::ok(ownership),
        Err(err) => Response::err(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string()),
    }
}
