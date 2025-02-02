use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::dtos::responses::RunepoolInterval;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]

pub struct Runepool {
    pub count: i64,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub units: i64,
}

impl From<RunepoolInterval> for Runepool {
    fn from(value: RunepoolInterval) -> Self {
        Self {
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
