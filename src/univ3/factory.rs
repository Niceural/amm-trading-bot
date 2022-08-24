use ethers::{
    prelude::{SignerMiddleware},
    providers::{Http, Provider},
    signers::LocalWallet,
    abi::Abi,
    types::Address,
    contract::Contract,
};
use std::fs::read_to_string;
use crate::uniswap_v3::pool_immutables::PoolImmutables;

pub struct Factory {
    chain_id: u64,
    address: Address,
    pool_immutables: Vec<PoolImmutables>,
}

impl Factory {
    pub fn new(
        address: Address,
        provider: &SignerMiddleware<Provider<Http>, LocalWallet>
    ) -> Self {
        println!("Factory: creating factory...");

        // create Factory contract
        let abi_string: String = i_uniswap_v3_factory_abi();
        let abi: Abi = serde_json::from_str(&abi_string).expect("Failed to parse ABI from string");
        Contract::new(address, abi, provider);

        // get all token addresses
    }

}

pub fn i_uniswap_v3_factory_abi() -> String {
    read_to_string("src/uniswap_v3/data/IUniswapV3FactoryABI.json")
        .expect("ABI file not found")
        .parse()
        .expect("Failed to parse ABI")
}