#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod abi;
mod database;
mod http;
mod models;
mod routes;
mod state;

use dotenvy::dotenv;
use state::AppState;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!("📦 enstate.rs v{}", env!("CARGO_PKG_VERSION"));
    match env::var("REDIS_URL") {
        Ok(val) => println!("📦 redis v{}", val),
        Err(_e) => println!("REDIS_URL is not defined in the .env file"),
    }
    
    match env::var("RPC_URL") {
        Ok(val) => println!("📦 rpc v{}", val),
        Err(_e) => println!("RPC_URL is not defined in the .env file"),
    }

    let state = AppState::new().await;

    http::setup(state).listen(3000).await;
}
