use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use mongodb::Collection;
use paris::{error, info, success};
use serde::Serialize;
use tokio::time::{sleep, Duration};

use crate::{
    config::database::get_mongo_pool,
    dtos::responses::{RunepoolHistoryResponse, RunepoolInterval},
    error::AppError,
    model::rune_pool::Runepool,
};

#[derive(Clone)]
pub struct MongoPollRepository {
    pools: Collection<MongoRunePool>,
}

#[derive(Clone, Serialize)]
pub struct MongoRunePool {
    pub count: i64,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub units: i64,
}

impl From<Runepool> for MongoRunePool {
    fn from(value: Runepool) -> Self {
        Self {
            count: value.count,
            start_time: value.start_time,
            end_time: value.end_time,
            units: value.units,
        }
    }
}

impl MongoPollRepository {
    pub async fn new() -> Result<Self, AppError> {
        let conn = get_mongo_pool()
            .await
            .map_err(|_| AppError::new("Mongodb is not connected"))?;

        Ok(Self {
            pools: conn.collection("runepools"),
        })
    }

    pub async fn populate_mongo_db(&self) -> Result<(), AppError> {
        info!("Starting populating runepool table in mongodb");

        let timestamp = Utc::now().timestamp();
        let mut from: i64 = 1741958400; //Mar 14, 2025
        let mut count: i16 = 0;

        let mut final_data: Vec<RunepoolInterval> = Vec::new();

        while from < timestamp && count < 2000 {
            let url = format!(
            "https://midgard.ninerealms.com/v2/history/runepool?interval=5min&from={}&count=400",
            from
        );

            // println!("GET:: {} || {}", url, count);

            // Add delay between requests (500ms)
            sleep(Duration::from_millis(500)).await;

            // Get the response
            let response = reqwest::get(&url).await.map_err(|e| {
                AppError::new(format!("Network error: {}", e)).with_status(StatusCode::BAD_GATEWAY)
            })?;

            if !response.status().is_success() {
                let status = response.status();
                let error_body = response
                    .text()
                    .await
                    .map_err(|e| AppError::new(format!("Failed to read error response: {}", e)))?;
                return Err(
                    AppError::new(format!("HTTP {} error: {}", status, error_body))
                        .with_status(StatusCode::BAD_GATEWAY),
                );
            }

            // Parse JSON response
            let parsed_data = response
                .json::<RunepoolHistoryResponse>()
                .await
                .map_err(|e| AppError::new(format!("Failed to parse runepool response: {}", e)))?;

            final_data.extend(parsed_data.intervals);
            let idx = final_data.len() - 1;
            from = final_data[idx]
                .end_time
                .parse()
                .expect("Not a valid string");

            count += 400;
        }

        let runepools: Vec<MongoRunePool> = final_data
            .iter()
            .cloned()
            .map(Runepool::from)
            .map(MongoRunePool::from)
            .collect();

        info!("Removing old data from mongodb...");
        let _ = self.pools.drop().await.map_err(|e| {
            error!("Error dropping collection {:#?}", e.to_string());
        });

        info!("Inserting data into mongodb...");

        //Insert the data into the database
        let start_time = std::time::Instant::now();

        match self.pools.insert_many(runepools.clone()).await {
            Ok(_) => {
                let elapsed = start_time.elapsed();
                success!(
                    "Mongodb populated successfully! Inserted {} records in {:.5} seconds",
                    runepools.len(),
                    elapsed.as_secs_f64()
                );
                Ok(())
            }
            Err(e) => {
                Err(AppError::new(e.to_string()).with_status(StatusCode::INTERNAL_SERVER_ERROR))
            }
        }
    }

    pub async fn read_mongodb(&self) -> Result<(), AppError> {
        let start_time = std::time::Instant::now();

        match self.pools.find(mongodb::bson::doc! {}).await {
            Ok(_) => {
                let elapsed = start_time.elapsed();
                success!("Read 2000 rows in {:.5}s", elapsed.as_secs_f64());
                Ok(())
            }
            Err(e) => Err(AppError::new(e.to_string())),
        }
    }
}
