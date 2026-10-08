#![forbid(unsafe_code)]

use std::{env, error::Error, time::Duration};

use frontal::Frontal;
use serde_json::Value;
use tokio::{signal, time};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Frontal::from_env()?;
    let period = poll_period()?;
    let mut ticker = time::interval(period);

    println!(
        "polling Frontal agent health every {} seconds",
        period.as_secs()
    );
    loop {
        tokio::select! {
            _ = ticker.tick() => match client.agents().get::<Value>("agents/health").await {
                Ok(health) => println!("agent health: {health}"),
                Err(error) => eprintln!("health request failed: {error}"),
            },
            result = signal::ctrl_c() => {
                result?;
                println!("shutting down");
                break;
            }
        }
    }

    Ok(())
}

fn poll_period() -> Result<Duration, Box<dyn Error>> {
    let seconds = env::var("FRONTAL_POLL_SECONDS")
        .unwrap_or_else(|_| "30".to_owned())
        .parse::<u64>()?;
    if seconds == 0 {
        return Err("FRONTAL_POLL_SECONDS must be greater than zero".into());
    }
    Ok(Duration::from_secs(seconds))
}
