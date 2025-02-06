use chrono::Utc;
use cron::Schedule;
use std::str::FromStr;
use std::thread;

use crate::client::proxy;

pub async fn run() {
    let expression = "0 */15 * * * *";
    let schedule = Schedule::from_str(expression).unwrap();

    loop {
        let now = Utc::now();
        let next = schedule.upcoming(Utc).next().unwrap();
        let duration = (next - now).to_std().unwrap();

        println!("Next job at: {}", next);
        thread::sleep(duration);

        println!("Running the scheduled data sync🔄...");
        // Your task here
        let res = proxy::sync_all_data().await;

        match res {
            Ok(_res) => println!("Data synced successfully ✅"),
            Err(_err) => println!("Failed ❌"),
        }
    }
}
