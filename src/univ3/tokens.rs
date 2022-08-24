use std::{path::Path, fs::File};

use serde::Deserialize;

// get assets list: https://app.uniswap.org/#/swap?chain=mainnet
// go to manage token list, settings, view list


// {
//   "name": "Uniswap Labs Default",
//   "timestamp": "2022-06-29T15:57:01.868Z",
//   "version": {
//     "major": 4,
//     "minor": 1,
//     "patch": 0
//   },
//   "tags": {},
//   "logoURI": "ipfs://QmNa8mQkrNKp1WEEeGjFezDmDeodkWRevGFN8JCV7b4Xir",
//   "keywords": ["uniswap", "default"],
//   "tokens": [

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Token {
    pub chain_id: u32,
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

pub fn get_tokens(chain_id: u32) -> Vec<Token> {
    let path = Path::new("src/utils/tokens.json");
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