//! List agents using a key from the environment.
//!
//! Run with `FRONTAL_API_KEY=frt_... cargo run --example list_agents`.

use frontal_sdk::{Client, Result};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::from_env()?;
    let agents: Value = client.get("agents").await?;
    println!(
        "{}",
        serde_json::to_string_pretty(&agents).expect("JSON value serializes")
    );
    Ok(())
}
