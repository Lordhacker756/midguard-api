#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RunepoolInterval {
    pub count: String,
    pub startTime: String,
    pub endTime: String,
    pub units: String,
}
