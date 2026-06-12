// https://leetcode.com/problems/number-of-ways-to-assign-edge-weights-ii

struct LCA {
    up: Vec<Vec<usize>>,
    depth: Vec<usize>,
    max_log: usize,
}

impl LCA {
    fn new(root: usize, adj: &Vec<Vec<usize>>) -> Self {
        let n = adj.len();
        let max_log = (n as f64).log2().ceil() as usize + 1;
        let mut up = vec![vec![root; max_log]; n];
        let mut depth = vec![0; n];

        Self::dfs(root, root, 0, adj, &mut up, &mut depth);

        // Fill binary lifting table
        for i in 1..max_log {
            for v in 0..n {
                let mid = up[v][i - 1];
                up[v][i] = up[mid][i - 1];
            }
        }

        LCA { up, depth, max_log }
    }

    fn dfs(
        u: usize,
        p: usize,
        d: usize,
        adj: &Vec<Vec<usize>>,
        up: &mut Vec<Vec<usize>>,
        depth: &mut Vec<usize>,
    ) {
        depth[u] = d;
        up[u][0] = p;
        for &v in &adj[u] {
            if v != p {
                Self::dfs(v, u, d + 1, adj, up, depth);
            }
        }
    }

    fn get_lca(&self, mut u: usize, mut v: usize) -> usize {
        if self.depth[u] < self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }

        // 1. Lift u to the same depth as v
        let diff = self.depth[u] - self.depth[v];
        for i in 0..self.max_log {
            if (diff >> i) & 1 == 1 {
                u = self.up[u][i];
            }
        }

        if u == v {
            return u;
        }

        // 2. Lift both until they are just below the LCA
        for i in (0..self.max_log).rev() {
            if self.up[u][i] != self.up[v][i] {
                u = self.up[u][i];
                v = self.up[v][i];
            }
        }

        self.up[u][0]
    }
}

struct Solution;

impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        static M: usize = 1_000_000_000 + 7;

        let n = edges.len() + 1;
        let edges = edges.into_iter().fold(vec![vec![]; n], |mut edges, edge| {
            let [v, u] = [edge[0] as usize - 1, edge[1] as usize - 1];

            edges[v].push(u);
            edges[u].push(v);
            edges
        });

        let mut dp = vec![[0, 0]; n];
        dp[0][0] = 1;
        for i in 1..n {
            dp[i][0] += dp[i - 1][0] + dp[i - 1][1];
            dp[i][0] %= M;
            dp[i][1] += dp[i - 1][0] + dp[i - 1][1];
            dp[i][1] %= M;
        }

        let lca = LCA::new(0, &edges);

        queries
            .into_iter()
            .map(|q| {
                let [v, u] = [q[0] as usize - 1, q[1] as usize - 1];

                let parent = lca.get_lca(v, u);
                let dv = lca.depth[v] - lca.depth[parent];
                let du = lca.depth[u] - lca.depth[parent];

                dp[dv + du][1] as i32
            })
            .collect()
    }
}
