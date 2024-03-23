use ethers::{
    prelude::*,
    types::transaction::{eip2718::TypedTransaction, eip2930::AccessList},
};
use ethers_flashbots::FlashbotsMiddleware;
use eyre::Result;
use reqwest::Url;
use std::time::{SystemTime, UNIX_EPOCH};
use dotenv::dotenv;

// use subway_rs::{abi, banner, numeric, relayer, telemetry, uniswap, utils};

#[tokio::main]
async fn main() -> Result<()> {
    // Get the http provider for flashbots use
    let http_provider = utils::get_http_provider()?;

    // Create the websocket clieant
    let client = utils::create_websocket_client().await?;

    // Get the latest block
    let last_block = client
        .get_block(BlockNumber::Latest)
        .await?
        .unwrap()
        .number
        .unwrap();

    Ok(())
}