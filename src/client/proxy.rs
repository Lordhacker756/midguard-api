use crate::error::AppError;
use axum::http::StatusCode;
use chrono::Utc;
use tokio::time::{sleep, Duration};

use crate::{
    dtos::responses::{
        DepthPriceHistoryResponse, EarningHistoryResponse, EarningInterval, PriceDepthInterval,
        RunepoolHistoryResponse, RunepoolInterval, SwapHistoryResponse, SwapInterval,
    },
    model::{
        earning_history::EarningHistory, price_history::PriceHistory, rune_pool::Runepool,
        swap_history::SwapHistory,
    },
    service::{
        earning_history_service::EarningHistoryService, price_history_service::PriceHistoryService,
        run_pool_service::RunePoolService, swap_history_service::SwapHistoryService,
    },
};

pub async fn get_prev_2_months_price_history() -> Result<(), AppError> {
    let price_history_service = PriceHistoryService::new()?;
    let now = Utc::now();
    let timestamp = now.timestamp();
    let mut from = price_history_service
        .get_last_update_timestamp()
        .await
        .unwrap_or(1730419200);

    println!("Last updated timestamp:: {}", from);

    //todo: Get the last updated timestamp and use it as 'from'

    let mut final_data: Vec<PriceDepthInterval> = Vec::new();

    while from < timestamp {
        let url = format!(
        "https://midgard.ninerealms.com/v2/history/depths/BTC.BTC?interval=5min&count=400&from={}",
        from
    );
        println!("GET:: {}", url);
        println!(
            "Current Timestamp:: {} | Limit Timestamp:: {} | Is it less:: {}",
            from,
            timestamp,
            from < timestamp
        );
        let res = reqwest::get(&url)
            .await
            .map_err(|e| AppError::new(format!("Failed to fetch price history: {}", e)))?
            .json::<DepthPriceHistoryResponse>()
            .await
            .map_err(|e| AppError::new(format!("Failed to parse price history response: {}", e)))?;

        final_data.extend(res.intervals);
        let idx = final_data.len() - 1;
        from = final_data[idx]
            .end_time
            .parse()
            .expect("Not a valid string")
    }
    println!(
        "DONE last timestamp @{} and total entries are {}",
        final_data[final_data.len() - 1].end_time,
        final_data.len()
    );

    //Convert the DTO to Db Model
    let price_history: Vec<PriceHistory> =
        final_data.iter().cloned().map(PriceHistory::from).collect();

    //Use the corresponding service to push the data to database
    let ids = price_history_service
        .save_batch(&price_history)
        .await
        .map_err(|e| AppError::new(format!("Failed to save price history: {}", e)))?;

    println!("{} rows inserted ✅", ids.len());
    Ok(())
}

pub async fn get_prev_2_months_earning_history() -> Result<(), AppError> {
    let earning_history_service = EarningHistoryService::new()?;
    let now = Utc::now();
    let timestamp = now.timestamp();
    let mut from = earning_history_service
        .get_last_update_timestamp()
        .await
        .unwrap_or(1730419200);

    println!("Getting earning history from :: {}", from);

    let mut final_data: Vec<EarningInterval> = Vec::new();

    while from < timestamp {
        let url = format!(
            "https://midgard.ninerealms.com/v2/history/earnings?interval=5min&from={}&count=400",
            from
        );

        println!("GET:: {}", url);
        println!(
            "Current Timestamp:: {} | Limit Timestamp:: {} | Is it less:: {}",
            from,
            timestamp,
            from < timestamp
        );
        let res = reqwest::get(&url)
            .await
            .map_err(|e| AppError::new(format!("Failed to fetch earning history: {}", e)))?
            .json::<EarningHistoryResponse>()
            .await
            .map_err(|e| {
                AppError::new(format!("Failed to parse earning history response: {}", e))
            })?;
        final_data.extend(res.intervals);
        let idx = final_data.len() - 1;
        from = final_data[idx]
            .end_time
            .parse()
            .expect("Not a valid string")
    }

    println!(
        "DONE last timestamp @{} and total entries are {}",
        final_data[final_data.len() - 1].end_time,
        final_data.len()
    );

    let earning_histories: Vec<EarningHistory> = final_data
        .iter()
        .cloned()
        .map(EarningHistory::from)
        .collect();

    let res = earning_history_service
        .save_batch(&earning_histories)
        .await
        .map_err(|e| AppError::new(format!("Failed to save earning history: {}", e)))?;

    println!("{} rows inserted ✅", res.len());
    Ok(())
}

pub async fn get_prev_2_months_swap_history() -> Result<(), AppError> {
    let swap_history_service = SwapHistoryService::new()?;
    let now = Utc::now();
    let timestamp = now.timestamp();
    let mut from = swap_history_service
        .get_last_update_timestamp()
        .await
        .unwrap_or(1730419200);

    println!("Fetching swap history form :: {}", from);
    let mut final_data: Vec<SwapInterval> = Vec::new();

    while from < timestamp {
        let url = format!(
            "https://midgard.ninerealms.com/v2/history/swaps?interval=5min&from={}&count=400",
            from
        );

        println!("GET:: {}", url);
        println!(
            "Current Timestamp:: {} | Limit Timestamp:: {} | Is it less:: {}",
            from,
            timestamp,
            from < timestamp
        );
        let res = reqwest::get(&url)
            .await
            .map_err(|e| AppError::new(format!("Failed to fetch swap history: {}", e)))?
            .json::<SwapHistoryResponse>()
            .await
            .map_err(|e| AppError::new(format!("Failed to parse swap history response: {}", e)))?;

        final_data.extend(res.intervals);
        let idx = final_data.len() - 1;
        from = final_data[idx]
            .end_time
            .parse()
            .expect("Not a valid string")
    }

    let swap_histories: Vec<SwapHistory> =
        final_data.iter().cloned().map(SwapHistory::from).collect();

    let _res = swap_history_service
        .save_batch(&swap_histories)
        .await
        .map_err(|e| AppError::new(format!("Failed to save swap history: {}", e)))?;

    println!("Swap History Synced ✅");
    Ok(())
}

pub async fn get_prev_2_months_runepool_history() -> Result<(), AppError> {
    let runepool_service = RunePoolService::new()?;
    let now = Utc::now();
    let timestamp = now.timestamp();
    let mut from = runepool_service
        .get_last_update_timestamp()
        .await
        .unwrap_or(1730419200);

    println!("Fetching runepool history from :: {}", from);
    let mut final_data: Vec<RunepoolInterval> = Vec::new();

    while from < timestamp {
        let url = format!(
            "https://midgard.ninerealms.com/v2/history/runepool?interval=5min&from={}&count=400",
            from
        );

        println!("GET:: {}", url);

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
    }

    let runepools: Vec<Runepool> = final_data.iter().cloned().map(Runepool::from).collect();

    runepool_service
        .save_batch(&runepools)
        .await
        .map_err(|e| AppError::new(format!("Failed to save runepool data: {}", e)))?;

    println!("Runepool History Synced ✅");
    Ok(())
}

pub async fn sync_all_data() -> Result<(), AppError> {
    println!("\n\n=========Syncing Price History 🔄===========");
    get_prev_2_months_price_history().await?;

    println!("\n\n=========Syncing Earning History 🔄===========");
    get_prev_2_months_earning_history().await?;

    println!("\n\n=========Syncing Swap History 🔄===========");
    get_prev_2_months_swap_history().await?;

    println!("\n\n=========Syncing Runepool History 🔄===========");
    get_prev_2_months_runepool_history().await?;

    println!("\n\n=========All Endpoints Synced Successfully ✅===========");
    Ok(())
}
