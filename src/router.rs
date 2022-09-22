use axum::{
    extract::{Path, Query},
    routing::get,
    Json, Router,
};
use ethers::types::U256;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    common::defines::{ContractType, GameClient, NetworkType},
    contracts::{erc20::ERC20, neco_nft::NecoNFT, neco_stake::NecoStake},
    models::{ERC20Token, NECOStakedInfo, NecoNFTMetadata, NecoNFTOwnership},
};

pub fn new_router() -> Router {
    Router::new()
        .route("/ping", get(ping))
        .route(
            "/neco-staked-info/:network/:public_address",
            get(get_neco_staked_info),
        )
        .route("/nft/ownership", get(get_nft_ownership))
        .route("/nft/metadata/:network/:nft_id", get(get_nft_metadata))
        .route("/erc20/balance", get(get_erc20_balance))
}

async fn ping() -> &'static str {
    "pong"
}

#[derive(Debug, Deserialize)]
pub struct GetErc20BalanceRequest {
    network: u8,
    contract_type: String,
    public_address: String,
}

async fn get_erc20_balance(
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

async fn get_nft_metadata(
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

#[derive(Debug, Deserialize)]
pub struct GetNftOwnershipRequest {
    network: u8,
    game_client: u8,
    public_address: String,
}

// get nft ownership by public address
async fn get_nft_ownership(
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

// get neco staked info by public address
async fn get_neco_staked_info(
    Path((network, public_address)): Path<(u8, String)>,
) -> Json<NecoResult<NECOStakedInfo>> {
    println!("public_address: {}", public_address);
    println!("network: {:?}", network);
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
    let staked_amount = NecoStake::new(network)
        .get_neco_staked_amount(&public_address)
        .await
        .unwrap_or_else(|_| U256::from(0));
    let staked_time = NecoStake::new(network)
        .get_neco_staked_time(&public_address)
        .await
        .unwrap_or_else(|_| U256::from(0));

    Json(NecoResult {
        status: StatusCode::OK.as_u16(),
        message: "network type error".to_string(),
        data: Some(NECOStakedInfo {
            public_address,
            staked_amount: staked_amount.to_string(),
            staked_time: staked_time.to_string(),
        }),
    })
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NecoResult<T: Serialize> {
    status: u16,
    message: String,
    data: Option<T>,
}

impl<T: Serialize> NecoResult<T> {
    pub fn new(status: StatusCode, message: String, data: Option<T>) -> Self {
        Self {
            status: status.as_u16(),
            message,
            data,
        }
    }
}
