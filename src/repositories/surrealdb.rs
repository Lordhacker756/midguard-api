use std::time::Instant;

use axum::http::StatusCode;
use chrono::Utc;
use paris::{info, success};
use surrealdb::{engine::remote::ws::Client, Surreal};
use tokio::time::{sleep, Duration, Interval};

use crate::{
    config::database::get_surreal_db,
    dtos::responses::{RunepoolHistoryResponse, RunepoolInterval},
    error::AppError,
};

pub struct SurrealRepository {
    db: &'static Surreal<Client>,
}

impl SurrealRepository {
    pub async fn new() -> Self {
        Self {
            db: get_surreal_db().await,
        }
    }

    // Get the data from midguard and insert into the database
    pub async fn insert_into_surreal_db(&self) -> Result<(), AppError> {
        info!("Starting populating runepool table in surreal db");

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

        info!("Dropping the runepools table if it exists");
        //Insert the data into the database
        let start_time = std::time::Instant::now();

        info!("Inserting data into surrealdb...");
        let _ = self.db.query("REMOVE TABLE runepools").await.unwrap();

        let res: Result<Vec<RunepoolInterval>, ()> = self
            .db
            .insert("runepools")
            .content(final_data)
            .await
            .map_err(|e| {
                AppError::new(e.to_string());
            });

        let end_time = start_time.elapsed();

        success!("Inserted 2000 records in {:.5}s ", end_time.as_secs_f32());

        Ok(())
    }

    // Read the data from the db
    pub async fn read_from_surreal_db(&self) -> Result<(), AppError> {
        info!("Reading from surreal db");
        let start_time = Instant::now();
        let res: Result<Vec<RunepoolInterval>, AppError> = self
            .db
            .select("runepools")
            .await
            .map_err(|e| AppError::new(e.to_string()));
        let end_time = start_time.elapsed();

        match res {
            Ok(data) => {
                success!(
                    "Read {} records from surrealDB in {:.5}s ",
                    data.len(),
                    end_time.as_secs_f32()
                );
            }
            Err(_) => {}
        }

        Ok(())
    }
}
