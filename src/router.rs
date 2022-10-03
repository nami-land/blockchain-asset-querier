use crate::apis::request::request_model::GetNECOStakedInfoRequest;
use crate::apis::{response, v1};
use crate::{
    apis::response::response_model::Response,
    apis::response::response_model::{
        ERC1155MetadataResponse, ERC1155OwnershipResponse, ERC20TokenResponse, ErrorResponse,
        NECOStakedInfoResponse,
    },
    common::defines::NetworkType,
    models::{
        ERC20Token, EmptyData, NECOStakedInfo, NecoNFTMetadata, NecoNFTOwnership, NecoNFTTrait,
        OwnershipItem,
    },
    services::neco_stake::NecoStakeService,
};
use axum::{routing::get, Router};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

pub fn new_router() -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui/*tail").url("/api-doc/openapi.json", ApiDoc::openapi()))
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
        (name = "Blockchain-Asset-Observer", description = "Todo items management API")
    )
)]
struct ApiDoc;
