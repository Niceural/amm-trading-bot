const TWO: f64 = 2.;
const TEN: f64 = 10.;

//------------------------------------- Exchange

use ethers::types::U256;

pub struct Exchange {
    pub token_0: usize, 
    pub token_1: usize,
    pub pool_id: usize,
    pub log_price: f64,
}

impl Exchange {
    pub fn new(
        token_0: usize,
        token_1: usize,
        pool_id: usize,
        sqrt_price_X96: U256,
        token_0_decimals: u8,
        token_1_decimals: u8,
    ) -> Self {
        Self {
            token_0,
            token_1,
            pool_id,
            log_price: 
                Exchange::sqrtPriceX96_to_log_price(sqrt_price_X96, token_0_decimals, token_1_decimals),
        }
    }

    pub fn log_price(&self) -> f64 {
        self.log_price
    }

    fn sqrtPriceX96_to_log_price(
        sqrt_price_X96: U256,
        token_0_decimals: u8,
        token_1_decimals: u8,
    ) -> f64 {
        let num = sqrt_price_X96.as_u128() as f64;
        - (num.powi(2) / TWO.powi(192) * TEN.powi((token_0_decimals - token_1_decimals) as i32)).ln()
    }
}

//------------------------------------- Arbitrage

pub struct Arbitrage {
    pub pool_count: usize,
    pub token_count: usize,
    pub adjacency_list: Vec<Exchange>,
}

impl Arbitrage {
    pub fn new(pool_count: usize, token_count: usize) -> Self {
        Self {
            pool_count,
            token_count,
            adjacency_list: Vec::with_capacity(pool_count),
        }
    }

    pub fn clear(&mut self) {
        self.adjacency_list.clear();
    }

    pub fn add_exchange(&mut self, exchange: Exchange) {
        self.adjacency_list.push(exchange);
    }

    pub fn execute(&self) {
        self.bellman_ford();
    }

    fn bellman_ford(&self) -> Vec<f64> {
        // initialize the distance to all vertices to infinity except start token
        let n = self.token_count;
        let mut dists = vec![f64::INFINITY; n];
        dists[self.adjacency_list[0].token_0] = 0.;
        let mut prev: Vec<Option<(usize, usize)>> = vec![None; n];
        let mut negative_cycles: Vec<Vec<(usize, usize)>> = Vec::new();

        // for each tokens, apply relaxation for all exchanges
        for _ in 0..(n-1) {
            for &Exchange { token_0, token_1, pool_id, log_price} in &self.adjacency_list {
                let new_dist = dists[token_0] + log_price;
                if new_dist < dists[token_1] {
                    dists[token_1] = new_dist;
                    prev[token_1] = Some((pool_id, token_0));
                }
            }
        }

        // if can still be relaxed => negative cycle
        for &Exchange { token_0, token_1, pool_id, log_price} in &self.adjacency_list {
            if dists[token_0] + log_price < dists[token_1] {
                let mut cycle = Vec::new();
                cycle.push((token_1, pool_id));
                cycle.push((token_0, pool_id));

                let mut current = prev[token_0].unwrap();
                loop {
                    match prev[current.0] {
                        Some(c) => {
                            cycle.push(c);
                            current = prev[c.0];
                        },
                        None => break,
                    }
                }
            }
        }

        dists
    }
}