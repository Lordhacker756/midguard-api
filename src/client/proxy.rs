use serde_json::Value;

pub async fn get_data() -> Result<(), reqwest::Error> {
    println!("hua");
    let body = reqwest::get(
        "https://midgard.ninerealms.com/v2/history/swaps?interval=hour&count=400&from=1606780800",
    )
    .await?
    .text()
    .await?;
    let res: Value = serde_json::from_str(&body).unwrap();
    println!("body = {res:?}");
    Ok(())
}
