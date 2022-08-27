use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::LocalWallet,
    types::{ Address, U256, },
    contract::Contract,
    abi::Abi,
};
use std::{
    path::Path,
    fs::{ File, },
    io::{Write, ErrorKind},
    collections::HashMap,
};
use serde::{Deserialize, Serialize};

use crate::utils::*;

//------------------------------------- Token

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Token {
    pub chain_id: u32,
    pub address: Address,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub token_id: usize,
}

impl Token {
    pub fn get_tokens(chain_id: u32) -> Vec<Token> {
        let file_storing_tokens = format!("config/{}/tokens.json", &chain_id);

        // create the file if file is not found
        let file = match File::open(&file_storing_tokens) {
            Ok(f) => f,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    panic!("Failed to open file: {}", &file_storing_tokens);
                    /*
                    println!("{} not found, creating from all tokens", &file_storing_tokens );
                    // reading all tokens from raw tokens file
                    let path_string = "config/raw/allTokens.json".to_string();
                    let path = Path::new(&path_string);
                    let error = format!("Failed to open file {}", &path_string);
                    let file = File::open(path).expect(&error);
                    let all_tokens: Vec<Token> = serde_json::from_reader(file).expect("Failed to extract all tokens from raw tokens file");
                    // storing tokens with correct chain id
                    let mut tokens: Vec<Token> = Vec::new();
                    let mut token_id: usize = 0;
                    for mut token in all_tokens {
                        if token.chain_id == chain_id {
                            token.token_id = token_id;
                            token_id += 1;
                            tokens.push(token);
                        }
                    }
                    // exporting tokens with correct chain id to file
                    let serialized_tokens = serde_json::to_string(&tokens).expect("Failed to serialize tokens");
                    let error = format!("Failed to create file {}", &file_storing_tokens );
                    let mut tokens_file = File::create(&file_storing_tokens ).expect(&error);
                    let error = format!("Failed to write tokens to file {}", &file_storing_tokens );
                    tokens_file.write_all(&serialized_tokens.as_bytes()).expect(&error);
                    return tokens;
                    */
                },
                _ => panic!("Failed to open file: {}", &file_storing_tokens),
            },
        };
        let tokens: Vec<Token> = serde_json::from_reader(file).expect("Failed to extract tokens from json");
        tokens
    }
}

//------------------------------------- PoolImmutables

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct PoolImmutables {
    pub address: Address,
    pub pool_id: usize,
    pub token_0_id: usize,
    pub token_1_id: usize,
    pub fee: f32,
    pub tick_spacing: f32,
    pub max_liquidity_per_tick: f32,
}

impl PoolImmutables {
    pub fn new(
        address: Address,
        pool_id: usize,
        token_0_id: usize,
        token_1_id: usize,
        fee: f32,
        tick_spacing: f32,
        max_liquidity_per_tick: f32
    ) -> Self {
        Self {
            address,
            pool_id,
            token_0_id,
            token_1_id,
            fee,
            tick_spacing,
            max_liquidity_per_tick,
        }
    }

    pub async fn get_pool_immutables(
        chain_id: u32,
        provider: &SignerMiddleware<Provider<Http>, LocalWallet>,
    ) -> Vec<PoolImmutables> {
        let file_storing_pools = format!("config/{}/pools.json", &chain_id);

        // create the file if file is not found
        let file = match File::open(&file_storing_pools) {
            Ok(f) => f,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    println!("{} not found, creating from UniswapV3Factory.getPool()", &file_storing_pools);

                    // get tokens
                    let path = format!("config/{}/tokens.json", &chain_id);
                    let error = format!("Failed to open file: {}", &path);
                    let path = Path::new(&path);
                    let tokens_file = File::open(path).expect(&error);
                    let tokens: Vec<Token> = serde_json::from_reader(tokens_file).expect("Failed to read Tokens from json");

                    // get factory contract
                    let factory_addr = univ3_factory_addr(chain_id);
                    let factory_abi: Abi = i_univ3_factory_abi();
                    let factory = Contract::new(factory_addr, factory_abi, provider);

                    // get pool abi
                    let pool_abi: Abi = i_univ3_pool_abi();

                    // fetch pool address
                    let mut pools: Vec<PoolImmutables> = Vec::new();
                    let mut pool_id: usize = 0;
                    let mut is_pool_fetched: HashMap<Address, bool> = HashMap::new();
                    let fees: Vec<u32> = vec![500, 3000, 10000];
                    for i in 0..tokens.len()-1 {
                    for j in i..tokens.len()-1 {
                    for fee in &fees {
                        let pool_addr: Address = factory
                            .method::<(Address, Address, u32), Address>("getPool", (tokens[i].address, tokens[j].address, fee.clone()))
                            .expect("`UniswapV3Factory.getPool()` method not found in ABI")
                            .call()
                            .await
                            .expect("`UniswapV3Factory.getPool()` asynchronous call failed");
                        // if pool exists and has not already been fetched
                        if pool_addr != Address::zero() && !is_pool_fetched.contains_key(&pool_addr) {
                            // add to hash map
                            is_pool_fetched.insert(pool_addr.clone(), true);
                            // get pool contract
                            let pool = Contract::new(pool_addr, pool_abi.clone(), &provider);
                            // get pool immutables
                            // find which is token 0 and is which token 1
                            let token0_addr: Address = pool
                                .method::<(), Address>("token0", ())
                                .expect("`Pool.token0()` method not found in ABI")
                                .call()
                                .await
                                .expect("`Pool.token0()` asynchronous call failed");
                            if token0_addr == tokens[i].address {
                                pools.push(PoolImmutables::new(
                                    pool_addr,
                                    pool_id,
                                    tokens[i].token_id,
                                    tokens[j].token_id,
                                    0., 0., 0.
                                ));
                            } else {
                                pools.push(PoolImmutables::new(
                                    pool_addr,
                                    pool_id,
                                    tokens[j].token_id,
                                    tokens[i].token_id,
                                    0., 0., 0.
                                ));
                            }
                            pool_id += 1;
                        }
                    }}}
                    // save to file
                    let serialized_pools = serde_json::to_string(&pools).expect("Failed to serialize pools");
                    let error = format!("Failed to create file {}", &file_storing_pools);
                    let mut pools_file = File::create(&file_storing_pools).expect(&error);
                    let error = format!("Failed to write pools to file {}", &file_storing_pools);
                    pools_file.write_all(&serialized_pools.as_bytes()).expect(&error);
                    return pools;
                },
                _ => panic!("Failed to open file: {}", &file_storing_pools),
            },
        };
        let pools: Vec<PoolImmutables> = serde_json::from_reader(file).expect("Failed to extract pool immutables from json");
        pools
    }
}

//------------------------------------- PoolState

pub struct PoolState {
    sqrt_price_X96: U256,
    tick: i32,
    observation_index: u16,
    observation_cardinality: u16,
    observation_cardinality_next: u16,
    fee_protocol: u8,
    unlocked: bool,
    token_0_decimals: u8,
    token_1_decimals: u8,
}

impl PoolState {
    pub fn new(
        (
            sqrt_price_X96,
            tick,
            observation_index,
            observation_cardinality,
            observation_cardinality_next,
            fee_protocol,
            unlocked,
        ): (U256, i32, u16, u16, u16, u8, bool),
        token_0_decimals: u8,
        token_1_decimals: u8
    ) -> Self {
        PoolState {
            sqrt_price_X96,
            tick,
            observation_index,
            observation_cardinality,
            observation_cardinality_next,
            fee_protocol,
            unlocked,
            token_0_decimals,
            token_1_decimals,
        }
    }
}