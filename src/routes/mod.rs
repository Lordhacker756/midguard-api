pub mod earning_history_route;
pub mod price_history_route;
pub mod rune_pool_route;
pub mod swap_history_route;

use axum::Json;
use crate::error::AppError;

pub type Result<T> = std::result::Result<T, AppError>;
pub type JsonResponse<T> = Result<Json<T>>;
