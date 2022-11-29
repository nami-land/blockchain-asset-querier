use axum::{extract::Query, Json};
use ethers::types::U256;
use reqwest::StatusCode;

use crate::{
    apis::{request::request_model::GetERC20BalanceRequest, response::response_model::Response},
    common::defines::{NetworkType, SupportedContractType},
    models::ERC20Token,
    services::erc20::ERC20Service,
};

#[utoipa::path(
    get,
    path = "/v1/erc20/balance",
    tag = "ERC20",
    params(
        GetERC20BalanceRequest
    ),
    responses(
        (status = 200, description = "Get ERC20 token balance successfully", body = ERC20TokenResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
    )
)]
pub async fn get_erc20_balance(
    Query(request): Query<GetERC20BalanceRequest>,
) -> Json<Response<ERC20Token>> {
    let network = match request.chain_id {
        1 => NetworkType::EthereumMainnet,
        5 => NetworkType::GoerliTestnet,
        _ => {
            return Response::err(StatusCode::BAD_REQUEST, "chain id is not supported");
        }
    };
    let contract_type = match request.contract_type.as_str() {
        "neco" => SupportedContractType::NAMIX,
        "nfish" => SupportedContractType::FISHX,
        _ => {
            return Response::err(StatusCode::BAD_REQUEST, "contract type error");
        }
    };

    let symbol = ERC20Service::new(contract_type, network)
        .get_symbol()
        .await
        .unwrap_or_else(|_| "unknown".to_string());
    let amount = ERC20Service::new(contract_type.clone(), network)
        .get_balance(&request.public_address)
        .await
        .unwrap_or_else(|_| U256::zero());
    let decimal = ERC20Service::new(contract_type.clone(), network)
        .get_decimal()
        .await
        .unwrap_or_else(|_| 0);

    Response::ok(ERC20Token {
        symbol,
        decimal,
        amount: amount.to_string(),
    })
}
