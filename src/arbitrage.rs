//------------------------------------- Pool

pub struct Pool {
    pub token_0: usize, 
    pub token_1: usize,
    pub pool_id: usize,
    pub log_price: f64,
    pub liquidity: f32,
    pub fee: u32,
}

impl Pool {
    pub fn new(
        token_0: usize,
        token_1: usize,
        pool_id: usize,
        log_price: f64,
        liquidity: f32,
        fee: u32
    ) -> Self {
        Self {
            token_0,
            token_1,
            pool_id,
            log_price,
            liquidity,
            fee,
        }
    }
}

//------------------------------------- Arbitrage

pub struct Arbitrage {
    pub pool_count: usize,
    pub token_count: usize,
    pub adjacency_list: Vec<Pool>,
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

    pub fn add_pool(&mut self, pool: Pool) {
        self.adjacency_list.push(pool);
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
            for &Pool { token_0, token_1, pool_id, log_price, .. } in &self.adjacency_list {
                let new_dist = dists[token_0] + log_price;
                if new_dist < dists[token_1] {
                    dists[token_1] = new_dist;
                    prev[token_1] = Some((pool_id, token_0));
                }
            }
        }

        // if can still be relaxed => negative cycle
        for &Pool { token_0, token_1, pool_id, log_price, liquidity, fee } in &self.adjacency_list {
            if dists[token_0] + log_price < dists[token_1] {
                let mut cycle = Vec::new();
                cycle.push((token_1, pool_id));
                cycle.push((token_0, pool_id));

                let mut current = token_0;
                loop {
                    match prev[current] {
                        Some(node) => {
                            cycle.push(node);
                            current = prev[node.0];
                        },
                        None => break,
                    }
                }
            }
        }

        dists
    }
}