use ethers::abi::Address;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::LocalWallet;
use std::convert::TryFrom;
use dotenv::dotenv;
use std::env;

mod utils; use utils::*;
use univ3::*;
mod arbitrage;

const LOG: bool = true;

#[tokio::main]
async fn main() {
    // dotenv and args config
    let args: Vec<String> = env::args().collect();
    dotenv().ok();
    let (chain_id, provider_url) = read_args(args);
    let secret_key = dotenv::var("SECRET_KEY_1").unwrap();
    println!("\n========== Trading Bot Started (chain id {}) ==========", &chain_id);

    // wallet and provider
    let wallet: LocalWallet = secret_key.parse().expect("Creating LocalWallet failed");
    let provider_service = Provider::<Http>::try_from(provider_url).expect("failed");
    let provider = SignerMiddleware::new(provider_service, wallet);

    // create json file with univ3 pools
    println!("Fetching Uniswap V3 pool addresses...");
    fetch_univ3_pools(chain_id, &provider).await;
}