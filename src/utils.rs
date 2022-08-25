use std::{
    path::Path,
    fs::{ File, read_to_string, }, io::Write,
};
use crate::{univ3::*, };
use ethers::{
    types::Address,
};

pub fn read_args(args: Vec<String>) -> (u32, String) {
    let network_name = &args[1];
    match network_name.as_str() {
        "ethereum" => (1, dotenv::var("MAINNET_RPC_URL").expect("Ethereum provider not found in .env")),
        "goerli" => (5, dotenv::var("GOERLI_RPC_URL").expect("Goerli provider not found in .env")),
        "optimism" => (10, dotenv::var("OPTIMISM_RPC_URL").expect("Optimism provider not found in .env")),
        "polygon" => (137, dotenv::var("POLYGON_RPC_URL").expect("Polygon provider not found in .env")),
        "arbitrum" => (42161, dotenv::var("ARBITRUM_RPC_URL").expect("Arbitrum provider not found in .env")),
        _ => panic!("Invalid CLI argument (network name)"),
    }
}

//------------------------------------- ABIs

pub fn i_univ3_factory_abi() -> String {
    let path = "src/config/univ3/IUniswapV3FactoryABI.json".to_string();
    let error = format!("File not found: {}", &path);
    read_to_string(&path)
        .expect(&error)
        .parse()
        .expect("Failed to parse ABI")
}

pub fn i_univ3_pool_abi() -> String {
    let path = "src/config/univ3/IUniswapV3PoolABI.json".to_string();
    let error = format!("File not found: {}", &path);
    read_to_string(&path)
        .expect(&error)
        .parse()
        .expect("Failed to parse ABI")
}

//------------------------------------- Contract addresses

pub fn univ3_factory_addr(chain_id: u32) -> Address {
    match chain_id {
        1 => "0x1F98431c8aD98523631AE4a59f267346ea31F984"
            .parse::<Address>()
            .expect("failed to parse address"),
        5 => "0x1F98431c8aD98523631AE4a59f267346ea31F984"
            .parse::<Address>()
            .expect("failed to parse address"),
        10 => "0x1F98431c8aD98523631AE4a59f267346ea31F984"
            .parse::<Address>()
            .expect("failed to parse address"),
        137 => "0x1F98431c8aD98523631AE4a59f267346ea31F984"
            .parse::<Address>()
            .expect("failed to parse address"),
        42161 => "0x1F98431c8aD98523631AE4a59f267346ea31F984"
            .parse::<Address>()
            .expect("failed to parse address"),
        _ => panic!("Invalid chain id"),
    }
}
