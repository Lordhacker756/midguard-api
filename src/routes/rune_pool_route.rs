use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::service::run_pool_service::RunePoolService;

pub async fn get_all_runepools() -> impl IntoResponse {
    let rune_service = RunePoolService::new();

    match rune_service.get_all_runepools().await {
        Ok(res) => Ok((StatusCode::OK, Json(res))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))),
    }
}
