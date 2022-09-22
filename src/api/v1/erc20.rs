use axum::{extract::Query, Json};
use ethers::types::U256;
use reqwest::StatusCode;

use crate::{
    api::{request::request_model::GetErc20BalanceRequest, response::response_model::NecoResult},
    common::defines::{ContractType, NetworkType},
    contracts::erc20::ERC20,
    models::ERC20Token,
};

pub async fn get_erc20_balance(
    Query(request): Query<GetErc20BalanceRequest>,
) -> Json<NecoResult<ERC20Token>> {
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
    let contract_type = match request.contract_type.as_str() {
        "neco" => ContractType::NECO,
        "nfish" => ContractType::NFISH,
        "busd" => ContractType::BUSD,
        _ => {
            return Json(NecoResult {
                status: StatusCode::BAD_REQUEST.as_u16(),
                message: "contract type error".to_string(),
                data: None,
            });
        }
    };

    let symbol = ERC20::new(contract_type, network)
        .get_symbol()
        .await
        .unwrap_or_else(|_| "unknown".to_string());
    let amount = ERC20::new(contract_type.clone(), network)
        .get_balance(&request.public_address)
        .await
        .unwrap_or_else(|_| U256::zero());
    let decimal = ERC20::new(contract_type.clone(), network)
        .get_decimal()
        .await
        .unwrap_or_else(|_| 0);

    Json(NecoResult {
        status: StatusCode::OK.as_u16(),
        message: "success".to_string(),
        data: Some(ERC20Token {
            symbol,
            decimal,
            amount: amount.to_string(),
        }),
    })
}
