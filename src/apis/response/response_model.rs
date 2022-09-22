use axum::Json;
use reqwest::StatusCode;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct NecoResponse<T: Serialize> {
    pub status: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> NecoResponse<T> {
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
