use chrono::Utc;

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
    let now = Utc::now();
    let timestamp = now.timestamp();
    let mut from = 1730419200;

    //todo: Get the last updated timestamp and use it as 'from'

    let mut final_data: Vec<PriceDepthInterval> = Vec::new();

    while from < timestamp {
        let url = format!(
        "https://midgard.ninerealms.com/v2/history/depths/BTC.BTC?interval=hour&count=400&from={}",
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
    let price_history_service = PriceHistoryService::new();
    let ids = price_history_service.save_batch(&price_history).await;

    match ids {
        Ok(val) => println!("{} rows inserted ✅", val.len()),
        Err(e) => print!("Error occured {:#?}", e),
    }
    Ok(())
}

pub async fn get_prev_2_months_earning_history() -> Result<(), reqwest::Error> {
    let now = Utc::now();
    let timestamp = now.timestamp();
    let mut from = 1730419200;

    let mut final_data: Vec<EarningInterval> = Vec::new();

    while from < timestamp {
        let url = format!(
            "https://midgard.ninerealms.com/v2/history/earnings?interval=hour&from={}&count=400",
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

    let earning_history_service = EarningHistoryService::new();
    let res = earning_history_service.save_batch(&earning_histories).await;

    match res {
        Ok(val) => println!("{} rows inserted ✅", val.len()),
        Err(e) => print!("Error occured {:#?}", e),
    }

    Ok(())
}

pub async fn get_prev_2_months_swap_history() -> Result<(), reqwest::Error> {
    let now = Utc::now();
    let timestamp = now.timestamp();
    let mut from = 1730419200;

    let mut final_data: Vec<SwapInterval> = Vec::new();

    while from < timestamp {
        let url = format!(
            "https://midgard.ninerealms.com/v2/history/swaps?interval=hour&from={}&count=400",
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

    let swap_history_service = SwapHistoryService::new();
    let res = swap_history_service.save_batch(&swap_histories).await;

    match res {
        Ok(val) => println!("{} rows inserted ✅", val.len()),
        Err(e) => print!("Error occured {:#?}", e),
    }

    Ok(())
}

pub async fn get_prev_2_months_runepool_history() -> Result<(), reqwest::Error> {
    let now = Utc::now();
    let timestamp = now.timestamp();
    let mut from = 1730419200;

    let mut final_data: Vec<RunepoolInterval> = Vec::new();

    while from < timestamp {
        let url = format!(
            "https://midgard.ninerealms.com/v2/history/runepool?interval=hour&from={}&count=400",
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
            .json::<RunepoolHistoryResponse>()
            .await?;

        final_data.extend(res.intervals);
        let idx = final_data.len() - 1;
        from = final_data[idx]
            .end_time
            .parse()
            .expect("Not a valid string")
    }

    let runepool_histories: Vec<Runepool> =
        final_data.iter().cloned().map(Runepool::from).collect();

    let runepool_history_service = RunePoolService::new();
    let res = runepool_history_service
        .save_batch(&runepool_histories)
        .await;

    match res {
        Ok(val) => println!("{} rows inserted ✅", val.len()),
        Err(e) => print!("Error occured {:#?}", e),
    }

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
