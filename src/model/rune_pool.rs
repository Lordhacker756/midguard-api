use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::dtos::responses::RunepoolInterval;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]

pub struct Runepool {
    pub id: Option<i32>,
    pub count: i64,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub units: i64,
}

#[derive(Deserialize, Debug)]
pub struct QueryParams {
    pub interval: Option<String>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub count: Option<i32>,
    pub date_range: Option<String>,
    pub units_gt: Option<i64>,
    pub units_lt: Option<i64>,
    pub units_eq: Option<i64>,
}

impl From<RunepoolInterval> for Runepool {
    fn from(value: RunepoolInterval) -> Self {
        Self {
            id: None,
            count: value.count.parse().unwrap_or(0),
            start_time: Utc
                .timestamp_opt(value.start_time.parse().unwrap_or(0), 0)
                .unwrap(),
            end_time: Utc
                .timestamp_opt(value.end_time.parse().unwrap_or(0), 0)
                .unwrap(),
            units: value.units.parse().unwrap_or(0),
        }
    }
}
