use ethers::abi::Address;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::LocalWallet;
use std::convert::TryFrom;
use dotenv::dotenv;
use std::env;

mod utils; use utils::*;
mod arbitrage; use arbitrage::*;
mod univ3; use univ3::*;

pub struct Bot {
    chain_id: u32,
    // wallet: LocalWallet,
    provider: SignerMiddleware<Provider<Http>, LocalWallet>,
    tokens: Vec<Token>,
    pool_immutables: Vec<PoolImmutables>,
    arbitrage: Arbitrage,
}

impl Bot {
    pub async fn new(
        chain_id: u32,
        secret_key: String,
        provider_url: String,
    ) -> Self {
        println!("\tCreating provider...");
        let wallet: LocalWallet = secret_key.parse().expect("Failed to create local wallet");
        let provider_service = Provider::<Http>::try_from(provider_url).expect("Failed to get provider from url");
        let provider: SignerMiddleware<Provider<Http>, LocalWallet> = SignerMiddleware::new(provider_service, wallet);

        println!("\tGetting tokens and pool immutables...");
        let tokens = Token::get_tokens(chain_id);
        let pool_immutables = PoolImmutables::get_pool_immutables(chain_id);

        println!("\tGet arbitrage instance...");
        let arbitrage = Arbitrage::new(pool_immutables.len(), tokens.len());

        Self {
            chain_id,
            // wallet,
            provider,
            tokens,
            pool_immutables,
            arbitrage,
        }
    }

    pub async fn run(&self) {
        // clear arbitrage
        // get token prices and add them to arbitrage
        // execute arbitrage
        // prepare arguments for on chain call
        // call on chain contract
        // store logs in log file
    }
}

#[tokio::main]
async fn main() {
    // dotenv and args config
    let args: Vec<String> = env::args().collect();
    dotenv().ok();
    let (chain_id, provider_url) = read_args(args);
    let secret_key = dotenv::var("SECRET_KEY_1").unwrap();
    println!("\n========== Trading Bot Started (chain id {}) ==========", &chain_id);

    // create bot
    println!("Creating Bot instance...");
    let bot: Bot = Bot::new(chain_id, secret_key, provider_url).await;
    loop { bot.run(); }
}