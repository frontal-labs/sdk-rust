//! List functions using an API key from the environment.
//!
//! Run with `FRONTAL_API_KEY=frt_... cargo run --example list_functions`.

use frontal::{models::functions::FunctionListOptions, Frontal, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Frontal::from_env()?;
    let functions = client
        .functions()
        .list_functions(&FunctionListOptions {
            limit: Some(20),
            ..FunctionListOptions::default()
        })
        .await?;
    println!(
        "{}",
        serde_json::to_string_pretty(&functions).expect("JSON value serializes")
    );
    Ok(())
}
