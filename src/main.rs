use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Wallet},
    contract::Contract,
    abi::Abi,
    core::k256::ecdsa::SigningKey, types::U256
    // types::Address,
};
use std::convert::TryFrom;
use dotenv::dotenv;
use std::env;

mod utils; use utils::*;
mod arbitrage; use arbitrage::*;
mod univ3; use univ3::*;

pub struct Bot {
    chain_id: u32,
    provider: SignerMiddleware<Provider<Http>, LocalWallet>,
    tokens: Vec<Token>,
    pool_immutables: Vec<PoolImmutables>,
    // pool_contracts: Vec<Contract<&SignerMiddleware<Provider<Provider>, Wallet<SigningKey>>>>,
    arbitrage: Arbitrage,
}

impl Bot {
    pub async fn new(
        chain_id: u32,
        secret_key: String,
        provider_url: String,
    ) -> Self {
        println!("Creating provider...");
        let wallet: LocalWallet = secret_key.parse().expect("Invalid private key");
        let provider_service = Provider::<Http>::try_from(provider_url).expect("Invalid provider url");
        let provider: SignerMiddleware<Provider<Http>, LocalWallet> = SignerMiddleware::new(provider_service, wallet);

        println!("Getting tokens and pool immutables...");
        let tokens = Token::get_tokens(chain_id);
        let pool_immutables = PoolImmutables::get_pool_immutables(chain_id, &provider).await;

        // println!("Getting pool contracts...");
        // let pool_contracts = Vec::with_capacity(pool_immutables.len());
        // let pool_abi = i_univ3_pool_abi();
        // let pool_abi: Abi = serde_json::from_str(&pool_abi).expect("Failed to parse string to Abi");
        // for pool in &pool_immutables {
        //     let temp = Contract::new(&pool.address, &pool_abi, &provider);
        //     // pool_contracts.push(Contract::new(&pool.address, &pool_abi, &provider));
        // }

        println!("Getting arbitrage instance...");
        let arbitrage = Arbitrage::new(pool_immutables.len(), tokens.len());

        Self {
            chain_id,
            provider,
            tokens,
            pool_immutables,
            // pool_contracts,
            arbitrage,
        }
    }

    pub async fn run(&mut self) {
        // clear arbitrage
        self.arbitrage.clear();

        // get pool contracts
        let mut pool_contracts = Vec::with_capacity(self.pool_immutables.len());
        let pool_abi: Abi = i_univ3_pool_abi();
        for pool in &self.pool_immutables {
            pool_contracts.push(Contract::new(pool.address, pool_abi.clone(), &self.provider));
        }

        // fetch pool prices
        for (contract, immutables) in pool_contracts.iter().zip(&self.pool_immutables) {
            let (sqrt_price_X96, tick, observation_index, observation_cardinality, observation_cardinality_next, fee_protocol, unlocked):
                (U256, i32, u16, u16, u16, u8, bool) = contract
                .method::<(), (U256, i32, u16, u16, u16, u8, bool)>("slot0", ())
                .expect("`UniswapV3Pool.slot0()` not found in ABI. Incorrect ABI.")
                .call()
                .await
                .expect("`UniswapV3Pool.slot0()` asynchronous call failed.");
            self.arbitrage.add_exchange(Exchange::new(
                immutables.token_0_id,
                immutables.token_1_id,
                immutables.pool_id,
                sqrt_price_X96,
                self.tokens[immutables.token_0_id].decimals,
                self.tokens[immutables.token_1_id].decimals,
            ));
        }

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
    let mut bot: Bot = Bot::new(chain_id, secret_key, provider_url).await;
    bot.run().await;
    // loop { bot.run().await; }
}