use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    app::AppState, dto::{request::{count_viewer::CountViewerRequest, viewer_create::ViewerCreateRequest}, response::{api_response::ApiResponse, count_response::CountResponse}}, infrastructure::kafka::producer
};

pub async fn send(Json(payload): Json<ViewerCreateRequest>) -> impl IntoResponse {
    // println!("payload in handler: {:?}", payload);
    // Gọi producer::start một lần, bắt lỗi và xử lý

    let message = "sent message successfully";

    match producer::start(payload.clone()).await {
        Ok(result) => {
            let response = ApiResponse::success_response(
                StatusCode::OK.into(),
                message.to_string(),
                result,
            );
            response.into_response()
        }
        Err(err) => {
            let response =
                ApiResponse::<()>::error_response(StatusCode::BAD_REQUEST.into(), err.to_string());
            response.into_response()
        }
    }
}

pub async fn get_count(State(app_state): State<AppState>, Json(payload): Json<CountViewerRequest>) -> impl IntoResponse{

    let message = "get viewer count successfully";

    match app_state.counter_viewer_usecase.get_viewer_usecase(payload.channel).await {
        Ok(count) => {
            let response = ApiResponse::success_response(
                StatusCode::OK.into(),
                message.to_string(),
                count,
            );
            response.into_response()
        }
        Err(err) => {
            let response =
                ApiResponse::<CountResponse>::error_response(StatusCode::BAD_REQUEST.into(), err.to_string());
            response.into_response()
        }
    }
}