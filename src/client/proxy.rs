use std::error::Error;

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

pub async fn get_prev_2_months_price_history() -> Result<(), reqwest::Error> {
    let price_history_service = PriceHistoryService::new();
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
            .await?
            .json::<DepthPriceHistoryResponse>()
            .await?;

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
    let ids = price_history_service.save_batch(&price_history).await;

    match ids {
        Ok(val) => println!("{} rows inserted ✅", val.len()),
        Err(e) => print!("Error occured {:#?}", e),
    }
    Ok(())
}

pub async fn get_prev_2_months_earning_history() -> Result<(), reqwest::Error> {
    let earning_history_service = EarningHistoryService::new();
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
            .await?
            .json::<EarningHistoryResponse>()
            .await?;
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

    let res = earning_history_service.save_batch(&earning_histories).await;

    match res {
        Ok(val) => println!("{} rows inserted ✅", val.len()),
        Err(e) => print!("Error occured {:#?}", e),
    }

    Ok(())
}

pub async fn get_prev_2_months_swap_history() -> Result<(), reqwest::Error> {
    let swap_history_service = SwapHistoryService::new();
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
            .await?
            .json::<SwapHistoryResponse>()
            .await?;

        final_data.extend(res.intervals);
        let idx = final_data.len() - 1;
        from = final_data[idx]
            .end_time
            .parse()
            .expect("Not a valid string")
    }

    let swap_histories: Vec<SwapHistory> =
        final_data.iter().cloned().map(SwapHistory::from).collect();

    let res = swap_history_service.save_batch(&swap_histories).await;

    match res {
        Ok(val) => println!("Swap History Synced ✅"),
        Err(e) => print!("Error occured {:#?}", e),
    }

    Ok(())
}

pub async fn get_prev_2_months_runepool_history() -> Result<(), Box<dyn std::error::Error>> {
    let runepool_service = RunePoolService::new();
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
            eprintln!("Network error: {}", e);
            eprintln!("Error details: {:#?}", e.source().unwrap_or(&e));
            e
        })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await?;
            eprintln!("HTTP Error {} for {}", status, url);
            eprintln!("Response body: {}", error_body);
            return Err(format!("HTTP {} error: {}", status, error_body).into());
        }

        // Parse JSON response
        let parsed_data = response
            .json::<RunepoolHistoryResponse>()
            .await
            .map_err(|e| {
                eprintln!("JSON parsing failed: {}", e);
                eprintln!("Error chain: {:#?}", e.source());
                e
            })?;

        final_data.extend(parsed_data.intervals);
        let idx = final_data.len() - 1;
        from = final_data[idx]
            .end_time
            .parse()
            .expect("Not a valid string");
    }

    let runepools: Vec<Runepool> = final_data.iter().cloned().map(Runepool::from).collect();

    runepool_service.save_batch(&runepools).await.map_err(|e| {
        eprintln!("Database error: {}", e);
        eprintln!("Error details: {:#?}", e);
        e
    })?;

    println!("Runepool History Synced ✅");
    Ok(())
}

pub async fn sync_all_data() -> Result<(), reqwest::Error> {
    println!("\n\n=========Syncing Price History 🔄===========");
    get_prev_2_months_price_history().await.unwrap();
    println!("\n\n=========Syncing Earning History 🔄===========");
    get_prev_2_months_earning_history().await.unwrap();
    println!("\n\n=========Syncing Swap History 🔄===========");
    get_prev_2_months_swap_history().await.unwrap();
    println!("\n\n=========Syncing Runepool History 🔄===========");
    get_prev_2_months_runepool_history().await.unwrap();

    println!("\n\n=========All Endpoints Synced Successfully ✅===========");

    Ok(())
}
