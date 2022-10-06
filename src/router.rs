use std::collections::HashMap;
use std::env;

use crate::apis::v1;
use crate::{
    apis::response::response_model::{
        ERC1155MetadataResponse, ERC1155OwnershipResponse, ERC20TokenResponse, ErrorResponse,
        NECOStakedInfoResponse,
    },
    common::defines::NetworkType,
    models::{
        ERC20Token, EmptyData, NECOStakedInfo, NecoNFTMetadata, NecoNFTOwnership, NecoNFTTrait,
        OwnershipItem,
    },
};
use axum::{routing::get, Router};
use utoipa::openapi::Server;
use utoipa::{Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

pub fn new_router() -> Router {
    let router = Router::new()
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
        .route("/v1/erc20/balance", get(v1::erc20::get_erc20_balance));

    // add openapi support
    let env_args: HashMap<String, String> = env::vars().collect();
    let env = env_args.get("_env");
    match env {
        Some(_) => router.merge(
            SwaggerUi::new("/swagger-ui/*tail")
                .url("/api-doc/openapi.json", RemoteApiDoc::openapi())
                .url(
                    "/blockchain-asset-querier/api-doc/openapi.json",
                    RemoteApiDoc::openapi(),
                ),
        ),
        None => {
            return router.merge(
                SwaggerUi::new("/swagger-ui/*tail")
                    .url("/api-doc/openapi.json", LocalApiDoc::openapi()),
            );
        }
    }
}

#[utoipa::path(
    get,
    path = "/ping",
    tag = "ping",
    responses(
        (status = 200, description = "Ping the service", body = [String])
    )
)]
async fn ping() -> &'static str {
    "pong"
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&RemoteApiServer),
    paths(
        ping,
        v1::neco_stake::get_neco_staked_info,
        v1::erc20::get_erc20_balance,
        v1::neco_nft::get_nft_ownership,
        v1::neco_nft::get_nft_metadata
    ),
    components(
        schemas(
            NetworkType,
            EmptyData,
            ERC20Token,
            NecoNFTTrait,
            NecoNFTMetadata,
            OwnershipItem,
            NecoNFTOwnership,
            ErrorResponse,
            NECOStakedInfo,
            NECOStakedInfoResponse,
            ERC20TokenResponse,
            ERC1155OwnershipResponse,
            ERC1155MetadataResponse
        ),
    ),
    tags(
        (name = "Blockchain-Asset-Querier", description = "Blockchain assets API")
    )
)]
struct RemoteApiDoc;

struct RemoteApiServer;

impl Modify for RemoteApiServer {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.servers = Some(vec![Server::new("/blockchain-asset-querier")]);
    }
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&LocalApiServer),
    paths(
        ping,
        v1::neco_stake::get_neco_staked_info,
        v1::erc20::get_erc20_balance,
        v1::neco_nft::get_nft_ownership,
        v1::neco_nft::get_nft_metadata
    ),
    components(
        schemas(
            NetworkType,
            EmptyData,
            ERC20Token,
            NecoNFTTrait,
            NecoNFTMetadata,
            OwnershipItem,
            NecoNFTOwnership,
            ErrorResponse,
            NECOStakedInfo,
            NECOStakedInfoResponse,
            ERC20TokenResponse,
            ERC1155OwnershipResponse,
            ERC1155MetadataResponse
        ),
    ),
    tags(
        (name = "Blockchain-Asset-Querier", description = "Blockchain assets API")
    )
)]
struct LocalApiDoc;

struct LocalApiServer;

impl Modify for LocalApiServer {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.servers = Some(vec![Server::new("/")]);
    }
}
