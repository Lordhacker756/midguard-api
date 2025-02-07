use crate::{client::proxy, error::AppError};
use axum::http::StatusCode;
use chrono::Utc;
use cron::Schedule;
use std::str::FromStr;
use std::thread;

pub async fn run() -> Result<(), AppError> {
    let expression = "0 */1 * * * *";
    let schedule = Schedule::from_str(expression).map_err(|e| {
        AppError::new(format!("Invalid cron expression: {}", e))
            .with_status(StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    loop {
        println!("Starting the job");
        let now = Utc::now();
        let next = schedule
            .upcoming(Utc)
            .next()
            .ok_or_else(|| AppError::new("Failed to calculate next schedule time"))?;

        let duration = (next - now)
            .to_std()
            .map_err(|e| AppError::new(format!("Duration calculation error: {}", e)))?;

        println!("Running the scheduled data sync🔄...");
        match proxy::sync_all_data().await {
            Ok(_) => println!("Data synced successfully ✅"),
            Err(e) => println!("Failed ❌: {:#?}", e),
        }
        println!("Next job at: {}", next);
        thread::sleep(duration);
    }
}
