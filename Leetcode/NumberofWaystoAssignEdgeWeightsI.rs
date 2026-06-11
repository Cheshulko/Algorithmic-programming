// https://leetcode.com/problems/number-of-ways-to-assign-edge-weights-i

struct Solution;

impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
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

        let mut depth = vec![0; n];

        fn dfs(
            v: usize,
            p: i32,
            d: usize,
            edges: &Vec<Vec<usize>>,
            depth: &mut Vec<usize>,
            dp: &mut Vec<[usize; 2]>,
        ) {
            depth[v] = d;

            for &to in edges[v].iter() {
                if to as i32 == p {
                    continue;
                }

                dp[to][0] += dp[v][0] + dp[v][1];
                dp[to][0] %= M;
                dp[to][1] += dp[v][0] + dp[v][1];
                dp[to][1] %= M;

                dfs(to, v as i32, d + 1, edges, depth, dp);
            }
        }

        dfs(0, -1, 0, &edges, &mut depth, &mut dp);

        let ma_d = depth.iter().max().copied().unwrap();
        let i = depth
            .into_iter()
            .enumerate()
            .filter_map(|(i, d)| (d == ma_d).then_some(i))
            .next()
            .unwrap();

        dp[i][1] as i32
    }
}
