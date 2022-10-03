use crate::{
    common::defines::NetworkType,
    models::EmptyData,
    models::{ERC20Token, NECOStakedInfo, NecoNFTMetadata, NecoNFTOwnership},
    services::neco_stake::NecoStakeService,
};
use axum::Json;
use reqwest::StatusCode;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Default, Serialize, ToSchema)]
#[aliases(
    NECOStakedInfoResponse = Response<NECOStakedInfo>,
    ERC20TokenResponse = Response<ERC20Token>,
    ERC1155OwnershipResponse = Response<NecoNFTOwnership>,
    ERC1155MetadataResponse = Response<NecoNFTMetadata>,
    ErrorResponse = Response<EmptyData>
)]
pub struct Response<T: Serialize> {
    pub status: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize + ToSchema> Response<T> {
    pub fn err(status: StatusCode, message: &str) -> Json<Self> {
        Json(Self {
            status: status.as_u16(),
            message: message.to_string(),
            data: None,
        })
    }

    pub fn ok(data: T) -> Json<Self> {
        Json(Self {
            status: StatusCode::OK.as_u16(),
            message: "success".to_string(),
            data: data.into(),
        })
    }
}
