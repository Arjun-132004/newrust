pub mod user;

pub use user::*;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    // pub fn error(message: String) -> ApiResponse<()> {
    //     ApiResponse {
    //         success: false,
    //         data: None,
    //         message: Some(message),
    //     }
    // }
}