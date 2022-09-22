use reqwest::StatusCode;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct NecoResult<T: Serialize> {
    pub status: u16,
    pub message: String,
    pub data: Option<T>,
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
