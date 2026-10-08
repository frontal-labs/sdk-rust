//! List agents using a key from the environment.
//!
//! Run with `FRONTAL_API_KEY=frt_... cargo run --example list_agents`.

use frontal::{Frontal, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Frontal::from_env()?;
    let agents = client.agents().list_agents(&[]).await?;
    println!(
        "{}",
        serde_json::to_string_pretty(&agents).expect("JSON value serializes")
    );
    Ok(())
}
