use axum::{debug_handler, http::StatusCode, response::IntoResponse, Json};

use crate::service::price_history_service::PriceHistoryService;

#[debug_handler]
pub async fn get_price_depth_history() -> impl IntoResponse {
    let price_history_service = PriceHistoryService::new();

    match price_history_service.get_all_price_history().await {
        Ok(res) => Ok((StatusCode::OK, Json(res))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
