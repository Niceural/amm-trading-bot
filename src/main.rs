use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, },
    contract::Contract,
    abi::Abi,
    types::U256
};
use std::convert::TryFrom;
use dotenv::dotenv;
use std::env;

mod utils; use utils::*;
mod graph; use graph::*;
mod univ3; use univ3::*;

pub struct Bot {
    provider: SignerMiddleware<Provider<Http>, LocalWallet>,
    tokens: Vec<Token>,
    pool_immutables: Vec<PoolImmutables>,
}

impl Bot {
    pub async fn new(
        chain_id: u32,
        secret_key: String,
        provider_url: String,
    ) -> Self {
        println!("\n-------------------- create bot instance");
        println!("creating local wallet...");
        let wallet: LocalWallet = secret_key.parse().expect("Invalid secret key. Please check it does not begin with '0x'.");
        println!("creating provider...");
        let provider_service = Provider::<Http>::try_from(provider_url).expect("Invalid provider url.");
        let provider: SignerMiddleware<Provider<Http>, LocalWallet> = SignerMiddleware::new(provider_service, wallet);

        println!("getting tokens config...");
        let tokens = Token::get_tokens(chain_id);
        println!("getting pool immutables config...");
        let pool_immutables = PoolImmutables::get_pool_immutables(chain_id, &provider).await;

        Self {
            provider,
            tokens,
            pool_immutables,
        }
    }

    pub async fn execute(&self) {
        println!("\n--------------------- execute bot");

        // create pool contracts
        println!("creating pool contracts...");
        let mut pool_contracts = Vec::with_capacity(self.pool_immutables.len());
        let pool_abi: Abi = i_univ3_pool_abi();
        for pool in &self.pool_immutables {
            pool_contracts.push(Contract::new(pool.address, pool_abi.clone(), &self.provider));
        }

        loop {
            // create graph instance
            let mut graph = Graph::new(self.tokens.len());

            // fetch pool state, convert sqrtPriceX96 to token0Price/token1Price, add edge to graph
            for (contract, immutables) in pool_contracts.iter().zip(&self.pool_immutables) {
                // fetch pool state
                let (sqrt_price_x_96, _, _, _, _, _, _):
                    (U256, i32, u16, u16, u16, u8, bool) = contract
                    .method::<(), (U256, i32, u16, u16, u16, u8, bool)>("slot0", ())
                    .expect("`UniswapV3Pool.slot0()` not found in ABI. Incorrect ABI.")
                    .call()
                    .await
                    .expect("`UniswapV3Pool.slot0()` asynchronous call failed.");
                // convert sqrtPriceX96 to log price
                let (p0, p1) = sqrtPriceX86_to_log_price(
                    sqrt_price_x_96,
                    self.tokens[immutables.token_0_id].decimals,
                    self.tokens[immutables.token_1_id].decimals,
                );
                // add edge to graph
                graph.add_edge(
                    immutables.token_0_id, 
                    immutables.token_1_id,
                    p0,
                    immutables.pool_id,
                );
                graph.add_edge(
                    immutables.token_1_id, 
                    immutables.token_0_id,
                    p1,
                    immutables.pool_id,
                );
            }

            // execute bellman ford
            let res = graph.bellman_ford_cycles(0);
            for el in res {
                println!("{:?}", el);
            }

            break;
        }
    }
}

#[tokio::main]
async fn main() {
    // dotenv and args config
    let args: Vec<String> = env::args().collect();
    dotenv().ok();
    let (chain_id, provider_url) = read_args(args);
    let secret_key = dotenv::var("SECRET_KEY_1").unwrap();
    println!("\n-------------------- Trading Bot Started (chain id {})", &chain_id);

    let bot: Bot = Bot::new(chain_id, secret_key, provider_url).await;
    bot.execute().await;
}
