use axum::{http::{StatusCode}, response::{IntoResponse, Response}, Json};
use serde::Serialize;

#[derive(Serialize,)]
pub struct ApiResponse<T> where T: Serialize {
    status_code: u16,
    message: String,
    data: Option<T>,
}

impl<T> ApiResponse<T> where T: Serialize + Default {

    pub fn get_status_code(&self) -> u16 {
        self.status_code
    }

    pub fn set_status_code(&mut self, status_code: u16) -> &mut Self {
        self.status_code = status_code;
        self
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }

    pub fn set_message(&mut self, message: String) -> &mut Self {
        self.message = message;
        self
    }

    pub fn success_response(status_code: u16, message: String, data: T) -> Self {
        ApiResponse {
            status_code: status_code,
            message: message,
            data: Some(data),
        }
    }

    pub fn error_response(status_code: u16, message: String) -> Self {
        ApiResponse {
            status_code: status_code,
            message: message,
            data: None,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status_code)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}