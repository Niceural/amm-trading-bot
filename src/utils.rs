use std::{
    fs::read_to_string,
};
use ethers::{
    types::{Address, },
    abi::{Abi, },
    prelude::U256,
};

const TWO: f64 = 2.;
const TEN: f64 = 10.;

/// Reads CLI arguments and returns the parameters.
/// CLI argument is the network name.
/// Returns chain id and RPC provider url.
pub fn read_args(args: Vec<String>) -> (u32, String) {
    let network_name = &args[1];
    match network_name.as_str() {
        "mainnet" => (1, dotenv::var("MAINNET_RPC_URL").expect("Mainnet provider not found in .env")),
        "goerli" => (5, dotenv::var("GOERLI_RPC_URL").expect("Goerli provider not found in .env")),
        "optimism" => (10, dotenv::var("OPTIMISM_RPC_URL").expect("Optimism provider not found in .env")),
        "polygon" => (137, dotenv::var("POLYGON_RPC_URL").expect("Polygon provider not found in .env")),
        "arbitrum" => (42161, dotenv::var("ARBITRUM_RPC_URL").expect("Arbitrum provider not found in .env")),
        n => panic!("Invalid CLI argument {}. Must be one of 'mainnet', 'goerli', 'optimism', 'polygon', arbitrum'.", n),
    }
}

//------------------------------------- price conversion

/// Return the price to pass to a `Graph` instance.
#[allow(non_snake_case)]
pub fn sqrtPriceX86_to_log_price(
    sqrt: U256,
    decimals_0: u8,
    decimals_1: u8,
) -> (f64, f64) {
    let mut p0p1: f64 = 0.;
    if sqrt.bits() > 128 {
        for b in 0..32 {
            p0p1 += (sqrt.byte(b) as f64) * 255. * (b as f64);
        }
    } else {
        p0p1 = sqrt.as_u128() as f64;
    }
    p0p1 = p0p1.powi(2) / TWO.powi(192) * TEN.powi(decimals_0 as i32 - decimals_1 as i32);
    return (-p0p1.ln(), -(1./p0p1).ln());
}

//------------------------------------- ABIs

/// Returns the ABI of [IUniswapV3Factory](https://github.com/Uniswap/v3-core/blob/412d9b236a1e75a98568d49b1aeb21e3a1430544/contracts/interfaces/IUniswapV3Factory.sol).
pub fn i_univ3_factory_abi() -> Abi {
    let path = "config/univ3/IUniswapV3FactoryABI.json".to_string();
    let error = format!("File not found: {}", &path);
    let abi: String = read_to_string(&path)
        .expect(&error)
        .parse()
        .expect("Failed to parse ABI.");
    let error: String = format!("Failed to parse the content of {} to ABI. Please check format.", &path);
    serde_json::from_str(&abi).expect(&error)
}

/// Returns the ABI of [IUniswapV3Pool](https://github.com/Uniswap/v3-core/blob/412d9b236a1e75a98568d49b1aeb21e3a1430544/contracts/interfaces/IUniswapV3Pool.sol).
pub fn i_univ3_pool_abi() -> Abi {
    let path = "config/univ3/IUniswapV3PoolABI.json".to_string();
    let error = format!("File not found: {}", &path);
    let abi: String = read_to_string(&path)
        .expect(&error)
        .parse()
        .expect("Failed to parse ABI.");
    let error: String = format!("Failed to parse the content of {} to ABI. Please check format.", &path);
    serde_json::from_str(&abi).expect(&error)
}

//------------------------------------- Contract addresses

/// Returns the address of the deployed instance of `[UniswapV3Factory](https://docs.uniswap.org/protocol/reference/core/UniswapV3Factory)` on network with chain id `chain_id`.
/// Panics if `chain_id` is unknown.
pub fn univ3_factory_addr(chain_id: u32) -> Address {
    match chain_id {
        1 => "0x1F98431c8aD98523631AE4a59f267346ea31F984"
            .parse::<Address>()
            .expect("Failed to parse address."),
        5 => "0x1F98431c8aD98523631AE4a59f267346ea31F984"
            .parse::<Address>()
            .expect("Failed to parse address."),
        10 => "0x1F98431c8aD98523631AE4a59f267346ea31F984"
            .parse::<Address>()
            .expect("Failed to parse address."),
        137 => "0x1F98431c8aD98523631AE4a59f267346ea31F984"
            .parse::<Address>()
            .expect("Failed to parse address."),
        42161 => "0x1F98431c8aD98523631AE4a59f267346ea31F984"
            .parse::<Address>()
            .expect("Failed to parse address."),
        n => panic!("Unknown chain id {}.", n),
    }
}
