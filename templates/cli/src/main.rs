#![forbid(unsafe_code)]

use std::{env, error::Error, process};

use frontal_sdk::Client;
use serde_json::Value;

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("error: {error}");
        process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "help".to_owned());

    match command.as_str() {
        "list-agents" => {
            let client = Client::from_env()?;
            let agents: Value = client.get("agents").await?;
            println!("{}", serde_json::to_string_pretty(&agents)?);
        }
        "create-agent" => {
            let payload = args.next().ok_or("provide a JSON request body")?;
            let body: Value = serde_json::from_str(&payload)?;
            let client = Client::from_env()?;
            let created: Value = client.post("agents", &body).await?;
            println!("{}", serde_json::to_string_pretty(&created)?);
        }
        "help" | "--help" | "-h" => {
            println!("Usage:\n  frontal-cli list-agents\n  frontal-cli create-agent '<json>'");
        }
        other => return Err(format!("unknown command {other:?}; use --help").into()),
    }

    Ok(())
}
