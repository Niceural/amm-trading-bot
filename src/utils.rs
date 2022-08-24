pub fn read_args(args: Vec<String>) -> (u32, String) {
    let network_name = &args[1];

    // chain id
    match network_name.as_str() {
        "ethereum" => (1, dotenv::var("MAINNET_RPC_URL").unwrap()),
        "goerli" => (5, dotenv::var("GOERLI_RPC_URL").unwrap()),
        "optimism" => (10, dotenv::var("OPTIMISM_RPC_URL").unwrap()),
        "polygon" => (137, dotenv::var("POLYGON_RPC_URL").unwrap()),
        "arbitrum" => (42161, dotenv::var("ARBITRUM_RPC_URL").unwrap()),
        _ => panic!("Invalid network name argument"),
    }
}
