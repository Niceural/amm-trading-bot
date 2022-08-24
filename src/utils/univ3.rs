use std::{
    path::Path,
    fs::{ File, read_to_string, }, collections::HashMap
};
use ethers::{
    types::{H160, Address},
    middleware::SignerMiddleware,
    providers::{Provider, Http},
    signers::LocalWallet,
    abi::Abi,
    contract::Contract,
};
use serde::{Deserialize, Serialize};
use crate::LOG;

struct PoolImmutables {
    factory: String,
    token_0: Token,
    token_1: Token,
    fee: u32,
    tick_spacing: i32,
    max_liquidity_per_tick: u128,
}

pub async fn fetch_univ3_pools(
    chain_id: u32,
    provider: &SignerMiddleware<Provider<Http>, LocalWallet>,
) {
    println!("Generating Uniswap V3 config files...");

    // get factory contract
    println!("\tGetting UniswapV3Factory contract...");
    let abi_string: String = i_univ3_factory_abi();
    let abi: Abi = serde_json::from_str(&abi_string).expect("Failed to parse ABI from string ABI");
    let factory_addr = univ3_factory_addr(chain_id);
    let factory = Contract::new(factory_addr, abi, provider);

    // get token addresses
    println!("\tGetting token addresses...");
    let tokens = get_tokens(chain_id);

    // get pool immutables
    println!("\tGetting UniswapV3Pool immutables...");
    let mut is_pool_fetched: HashMap<Address, bool> = HashMap::new();
    let mut pools: Vec<PoolImmutables> = Vec::new();
    for t0 in &tokens {
        for t1 in &tokens {
            let t0_addr = t0.address.parse::<Address>().expect("failed to parse address");
            let t1_addr = t1.address.parse::<Address>().expect("failed to parse address");
            let pool_addr: Address = factory
                .method::<(Address, Address, u32), Address>("getPool", (t0_addr, t1_addr, 3000 as u32))
                .expect("`Factory.getPool` method failed")
                .call()
                .await
                .expect("asynchronous call failed");
            if pool_addr != Address::zero() {
                is_pool_fetched.insert(pool_addr, true);
            }
        }
    }

    // writing config to file
}

// ------------------------------------ Tokens

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct Token {
    pub chain_id: u32,
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

fn get_tokens(chain_id: u32) -> Vec<Token> {
    let path = Path::new("src/utils/univ3/allTokens.json");
    let file = File::open(path).expect("Tokens file not found");
    let all_tokens: Vec<Token> = serde_json::from_reader(file).expect("Failed to parse json.");
    let mut tokens: Vec<Token> = Vec::new();
    for token in all_tokens {
        if token.chain_id == chain_id {
            tokens.push(token);
        }
    }
    tokens
}

// ------------------------------------ ABIs

pub fn i_univ3_factory_abi() -> String {
    read_to_string("src/utils/univ3/IUniswapV3FactoryABI.json")
        .expect("ABI file not found")
        .parse()
        .expect("Failed to parse ABI")
}

pub fn i_univ3_pool_abi() -> String {
    read_to_string("src/utils/univ3/IUniswapV3PoolABI")
        .expect("ABI file not found")
        .parse()
        .expect("Failed to parse ABI")
}

// ------------------------------------ Contract addresses

pub fn univ3_factory_addr(chain_id: u32) -> Address {
    match chain_id {
        _ => "0x1F98431c8aD98523631AE4a59f267346ea31F984"
            .parse::<Address>()
            .expect("failed to parse address"),
    }
}