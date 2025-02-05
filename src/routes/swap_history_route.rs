use axum::{debug_handler, extract::Query, http::StatusCode, response::IntoResponse, Json};

use crate::{model::swap_history::QueryParams, service::swap_history_service::SwapHistoryService};

#[debug_handler]
pub async fn get_all_swap_history(params: Query<QueryParams>) -> impl IntoResponse {
    let swap_history_service = SwapHistoryService::new();

    match swap_history_service.get_all_swap_history(params).await {
        Ok(res) => Ok((StatusCode::OK, Json(res))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))),
    }
}
