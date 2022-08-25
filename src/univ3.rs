use ethers::types::Address;
use std::{
    path::Path,
    fs::{ File, },
    io::{Write, ErrorKind},
};
use serde::{Deserialize, Serialize};

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
        let file_storing_tokens = format!("src/config/{}/tokens.json", &chain_id);
        // create the file if file is not found
        let file = match File::open(&file_storing_tokens) {
            Ok(f) => f,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    println!("{} not found, creating from all tokens", &file_storing_tokens );
                    // reading all tokens from raw tokens file
                    let path_string = "src/config/raw/allTokens.json".to_string();
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
                    let error = format!("Failed to write to file {}", &file_storing_tokens );
                    tokens_file.write_all(&serialized_tokens.as_bytes()).expect(&error);
                    tokens_file
                },
                _ => panic!("Failed to open tokens file"),
            },
        };
        let tokens: Vec<Token> = serde_json::from_reader(file).expect("Failed to extract tokens from json");
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
        let file_storing_pools = format!("src/config/{}/pools.json", &chain_id);
        // create the file if file is not found
        let file = match File::open(&file_storing_pools) {
            Ok(f) => f,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    println!("{} not found, creating from Factory.getPool()", &file_storing_pools);
                    pools_file
                },
                _ => panic!("Failed to open pools file"),
            },
        };
        let pools: Vec<PoolImmutables> = serde_json::from_reader(file).expect("Failed to extract pool immutables from json");
        pools
    }
}