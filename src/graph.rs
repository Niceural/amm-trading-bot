//------------------------------------- Edge

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub to: usize,
    pub weight: f64,
    pub edge_id: usize,
}

impl Edge {
    pub fn new(
        to: usize,
        weight: f64,
        edge_id: usize,
    ) -> Self {
        Self { to, weight, edge_id, }
    }
}

//------------------------------------- Graph

pub struct Graph {
    pub inner: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Self { inner: vec![vec![]; n], }
    }

    /// Adds a directed edge to the graph from `u` to `v` with weight `weight`.
    pub fn add_edge(&mut self, u: usize, v: usize, weight: f64, edge_id: usize) {
        self.inner[u].push(Edge::new(v, weight, edge_id));
    }

    pub fn node_count(&self) -> usize {
        self.inner.len()
    }

    /// Iterates over all nodes in the graph
    pub fn nodes(&self) -> impl Iterator<Item = (usize, &Vec<Edge>)> {
        self.inner.iter().enumerate()
    }

    pub fn bellman_ford(&self, start: usize) -> Vec<f64> {
        // initialize the distance to all nodes to infinity except start node
        let n = self.node_count();
        let mut dists = vec![f64::INFINITY; n];
        dists[start] = 0.;

        // for each node apply relaxation for all the edges
        for _ in 1..n {
        for (from, edges) in self.nodes() {
        for &Edge { to, weight, edge_id } in edges {
            let new_weight = dists[from] + weight;
            if new_weight < dists[to] {
                dists[to] = new_weight;
            }
        }}}

        // if can still be relaxed => negative cycle
        for _ in 1..n {
        for (from, edges) in self.nodes() {
        for &Edge { to, weight, edge_id } in edges {
            if dists[from] + weight < dists[to] {
                dists[to] = f64::NEG_INFINITY;
            }
        }}}

        dists
    }

    pub fn bellman_ford_cycles(&self, start: usize) -> Vec<Vec<(usize, usize)>> {
        // initialize the distance to all nodes to infinity except start node
        let n = self.node_count();
        let mut dists = vec![f64::INFINITY; n];
        dists[start] = 0.;

        // edge to get to previous node in negative cycle
        let mut prev: Vec<Option<(usize, usize)>> = vec![None; n];
        let mut negative_cycles: Vec<Vec<(usize, usize)>> = Vec::new();

        // for each node apply relaxation for all the edges
        for _ in 1..n {
        for (from, edges) in self.nodes() {
        for &Edge { to, weight, edge_id } in edges {
            let new_weight = dists[from] + weight;
            if new_weight < dists[to] {
                dists[to] = new_weight;
                prev[to] = Some((from, edge_id));
            }
        }}}

        // if can still be relaxed => negative cycle
        for (from, edges) in self.nodes() {
        for &Edge { to, weight, edge_id } in edges {
            if dists[from] + weight < dists[to] {
                dists[to] = f64::NEG_INFINITY;
                let mut cycle: Vec<(usize, usize)> = Vec::new();
                cycle.push((to, edge_id)); cycle.push((from, edge_id));
                let mut curr = from;
                let mut nodes_count = 2;
                loop {
                    match prev[curr] {
                        Some(n) => {
                            if cycle.contains(&n) || nodes_count > 10 {
                                cycle.push(n);
                                cycle.reverse();
                                negative_cycles.push(cycle);
                                break;
                            }
                            cycle.push(n);
                            curr = n.0;
                            nodes_count += 1;
                        },
                        None => {
                            cycle.reverse();
                            negative_cycles.push(cycle);
                            break;
                        },
                    }
                }
            }
        }}

        negative_cycles
    }
}

//------------------------------------- tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bellman_ford_1() {
        let mut graph = Graph::new(9);

        graph.add_edge(0, 1, 1., 0);
        graph.add_edge(1, 2, 1., 0);
        graph.add_edge(2, 4, 1., 0);
        graph.add_edge(4, 3, -3., 0);
        graph.add_edge(3, 2, 1., 0);
        graph.add_edge(1, 5, 4., 0);
        graph.add_edge(1, 6, 4., 0);
        graph.add_edge(5, 6, 5., 0);
        graph.add_edge(6, 7, 4., 0);
        graph.add_edge(5, 7, 3., 0);

        let dists = graph.bellman_ford(0);

        assert_eq!(
            &dists,
            &[
                0.00,           // 0 -> 0
                1.00,           // 0 -> 1
                -f64::INFINITY, // 0 -> 2
                -f64::INFINITY, // 0 -> 3
                -f64::INFINITY, // 0 -> 4
                5.00,           // 0 -> 5
                5.00,           // 0 -> 6
                8.00,           // 0 -> 7
                f64::INFINITY,  // 0 -> 8
            ]
        );
    }

    #[test]
    fn test_bellman_ford_cycle_1() {
        let mut graph = Graph::new(6);

        graph.add_edge(0, 1, -f64::log(0.23, 2.), 0);
        graph.add_edge(0, 2, -f64::log(0.25, 2.), 0);
        graph.add_edge(0, 3, -f64::log(16.43, 2.), 0);
        graph.add_edge(0, 4, -f64::log(18.21, 2.), 0);
        graph.add_edge(0, 5, -f64::log(4.94, 2.), 0);
        
        graph.add_edge(1, 0, -f64::log(4.34, 2.), 0);
        graph.add_edge(1, 2, -f64::log(1.11, 2.), 0);
        graph.add_edge(1, 3, -f64::log(71.40, 2.), 0);
        graph.add_edge(1, 4, -f64::log(79.09, 2.), 0);
        graph.add_edge(1, 5, -f64::log(21.44, 2.), 0);

        graph.add_edge(2, 0, -f64::log(3.93, 2.), 0);
        graph.add_edge(2, 1, -f64::log(0.9, 2.), 0);
        graph.add_edge(2, 3, -f64::log(64.52, 2.), 0);
        graph.add_edge(2, 4, -f64::log(71.48, 2.), 0);
        graph.add_edge(2, 5, -f64::log(19.37, 2.), 0);

        graph.add_edge(3, 0, -f64::log(0.061, 2.), 0);
        graph.add_edge(3, 1, -f64::log(0.014, 2.), 0);
        graph.add_edge(3, 2, -f64::log(0.015, 2.), 0);
        graph.add_edge(3, 4, -f64::log(1.11, 2.), 0);
        graph.add_edge(3, 5, -f64::log(0.3, 2.), 0);

        graph.add_edge(4, 0, -f64::log(0.055, 2.), 0);
        graph.add_edge(4, 1, -f64::log(0.013, 2.), 0);
        graph.add_edge(4, 2, -f64::log(0.014, 2.), 0);
        graph.add_edge(4, 3, -f64::log(0.9, 2.), 0);
        graph.add_edge(4, 5, -f64::log(0.27, 2.), 0);

        graph.add_edge(5, 0, -f64::log(0.2, 2.), 0);
        graph.add_edge(5, 1, -f64::log(0.047, 2.), 0);
        graph.add_edge(5, 2, -f64::log(0.052, 2.), 0);
        graph.add_edge(5, 3, -f64::log(3.33, 2.), 0);
        graph.add_edge(5, 4, -f64::log(3.69, 2.), 0);

        let cycles = graph.bellman_ford_cycles(0);

        assert!(cycles.len() > 0);
        for c in &cycles {
            println!("{:?}", c);
        }
        assert!(cycles.contains(&vec![(3, 0), (4, 0), (0, 0), (3, 0)]));
    }
}