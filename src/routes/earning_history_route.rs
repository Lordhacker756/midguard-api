use axum::{debug_handler, extract::Query, http::StatusCode, response::IntoResponse, Json};

use crate::{
    model::earning_history::QueryParams, service::earning_history_service::EarningHistoryService,
};

#[debug_handler]
pub async fn get_all_earnings_history(params: Query<QueryParams>) -> impl IntoResponse {
    let earning_history_service = match EarningHistoryService::new() {
        Ok(service) => service,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    match earning_history_service
        .get_all_earnings_history(params)
        .await
    {
        Ok(res) => Ok((StatusCode::OK, Json(res))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
