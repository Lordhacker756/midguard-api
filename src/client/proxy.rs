use chrono::Utc;

use crate::dtos::responses::{
    DepthPriceHistoryResponse, EarningHistoryResponse, EarningInterval, PriceDepthInterval,
    RunepoolHistoryResponse, RunepoolInterval, SwapHistoryResponse, SwapInterval,
};

pub async fn get_prev_2_months_price_history() -> Result<Vec<PriceDepthInterval>, reqwest::Error> {
    let now = Utc::now();
    let timestamp = now.timestamp();
    let mut from = 1730419200;

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
    Ok(final_data)
}

pub async fn get_prev_2_months_earning_history() -> Result<Vec<EarningInterval>, reqwest::Error> {
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
    Ok(final_data)
}

pub async fn get_prev_2_months_swap_history() -> Result<Vec<SwapInterval>, reqwest::Error> {
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

    println!(
        "DONE last timestamp @{} and total entries are {}",
        final_data[final_data.len() - 1].end_time,
        final_data.len()
    );
    Ok(final_data)
}

pub async fn get_prev_2_months_runepool_history() -> Result<Vec<RunepoolInterval>, reqwest::Error> {
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

    println!(
        "DONE last timestamp @{} and total entries are {}",
        final_data[final_data.len() - 1].end_time,
        final_data.len()
    );
    Ok(final_data)
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
