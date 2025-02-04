use axum::{debug_handler, http::StatusCode, response::IntoResponse, Json};

use crate::service::earning_history_service::EarningHistoryService;

#[debug_handler]
pub async fn get_all_earnings_history() -> impl IntoResponse {
    let earning_history_service = EarningHistoryService::new();

    match earning_history_service.get_all_earnings_history().await {
        Ok(res) => Ok((StatusCode::OK, Json(res))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
