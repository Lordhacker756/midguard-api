use chrono::Utc;

use crate::model::responses::{DepthPriceHistoryResponse, Interval};

pub async fn get_prev_2_months_price_history() -> Result<Vec<Interval>, reqwest::Error> {
    let now = Utc::now();
    let timestamp = now.timestamp();
    let mut from = 1730419200;

    let mut final_data: Vec<Interval> = Vec::new();

    while from < timestamp {
        let url = format!(
        "https://midgard.ninerealms.com/v2/history/depths/BTC.BTC?interval=hour&&count=400&from={}",
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
        from = final_data[idx].endTime.parse().expect("Not a valid string")
    }
    println!(
        "DONE last timestamp @{} and total entries are {}",
        final_data[final_data.len() - 1].endTime,
        final_data.len()
    );
    Ok(final_data)
}
