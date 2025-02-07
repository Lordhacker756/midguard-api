use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};

use crate::{model::rune_pool::QueryParams, service::run_pool_service::RunePoolService};

pub async fn get_all_runepools(params: Query<QueryParams>) -> impl IntoResponse {
    let rune_service = match RunePoolService::new() {
        Ok(service) => service,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    match rune_service.get_all_runepools(params).await {
        Ok(res) => Ok((StatusCode::OK, Json(res))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
