#![forbid(unsafe_code)]

use std::{env, error::Error, net::SocketAddr};

use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use frontal_sdk::Client;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::from_env()?;
    let address = env::var("BIND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:3000".to_owned())
        .parse::<SocketAddr>()?;
    let app = routes().with_state(client);
    let listener = tokio::net::TcpListener::bind(address).await?;

    println!("listening on http://{address}");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

fn routes() -> Router<Client> {
    Router::new()
        .route("/healthz", get(health))
        .route("/agents", get(list_agents))
}

async fn health() -> &'static str {
    "ok"
}

async fn list_agents(State(client): State<Client>) -> Result<Json<Value>, (StatusCode, String)> {
    client.get("agents").await.map(Json).map_err(|error| {
        eprintln!("Frontal agent request failed: {error}");
        (
            StatusCode::BAD_GATEWAY,
            "upstream Frontal request failed".to_owned(),
        )
    })
}

async fn shutdown_signal() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        eprintln!("could not listen for shutdown signal: {error}");
    }
}
