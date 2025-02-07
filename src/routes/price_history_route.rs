use axum::{debug_handler, extract::Query, http::StatusCode, response::IntoResponse, Json};

use crate::{
    model::price_history::PriceHistoryParams, service::price_history_service::PriceHistoryService,
};

#[debug_handler]
pub async fn get_price_depth_history(params: Query<PriceHistoryParams>) -> impl IntoResponse {
    let price_history_service = match PriceHistoryService::new() {
        Ok(service) => service,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    match price_history_service.get_all_price_history(params).await {
        Ok(res) => Ok((StatusCode::OK, Json(res))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
