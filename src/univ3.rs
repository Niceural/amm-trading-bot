use ethers::types::Address;
use std::{
    path::Path,
    fs::{ File, read_to_string, },
};
use serde::{Deserialize, Serialize};

//------------------------------------- Token

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Token {
    pub chain_id: u32,
    pub addr: Address,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub token_id: usize,
}

impl Token {
    pub fn get_tokens(chain_id: u32) -> Vec<Token> {
        let path_string = format!("src/config/{}/tokens.json", chain_id);
        let path = Path::new(&path_string);
        let file = File::open(path).expect("Tokens file not found");
        let tokens: Vec<Token> = serde_json::from_reader(file).expect("Failed to convert json to tokens");
        tokens
    }
}

//------------------------------------- PoolImmutable

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct PoolImmutables {
    pub addr: Address,
    pub pool_id: usize,
    pub token_0_id: usize,
    pub token_1_id: usize,
    pub fee: f32,
    pub tick_spacing: f32,
    pub max_liquidity_per_tick: f32,
}

impl PoolImmutables {
    pub fn get_pool_immutables(chain_id: u32) -> Vec<PoolImmutables> {
        let path_string = format!("src/config/{}/pools.json", chain_id);
        let path = Path::new(&path_string);
        let file = File::open(path).expect("Pools file not found");
        let pools: Vec<PoolImmutables> = serde_json::from_reader(file).expect("Failed to convert json to pools");
        pools
    }
}