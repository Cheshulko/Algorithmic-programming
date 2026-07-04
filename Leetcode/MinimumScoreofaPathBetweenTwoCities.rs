// https://leetcode.com/problems/minimum-score-of-a-path-between-two-cities

struct Solution;

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        fn dfs(cur: usize, adj: &[Vec<(usize, i32)>], seen: &mut [bool]) -> i32 {
            seen[cur] = true;

            let mut mi = i32::MAX;
            for &(to, c) in adj[cur].iter() {
                mi = mi.min(c);
                if seen[to] {
                    continue;
                }
                mi = mi.min(dfs(to, adj, seen));
            }

            mi
        }

        let n = n as usize;
        let adj = roads.into_iter().fold(vec![vec![]; n], |mut adj, r| {
            let (u, v, c) = (r[0] as usize - 1, r[1] as usize - 1, r[2]);

            adj[u].push((v, c));
            adj[v].push((u, c));
            adj
        });

        let mut seen = vec![false; n];

        dfs(0, &adj, &mut seen)
    }
}
