// https://leetcode.com/problems/remove-methods-from-project

struct Solution;

impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        fn dfs(cur: usize, adj: &[Vec<usize>], seen: &mut [bool]) {
            seen[cur] = true;

            for &to in adj[cur].iter() {
                if !seen[to] {
                    dfs(to, adj, seen);
                }
            }
        }

        let (n, k) = (n as usize, k as usize);

        let adj = invocations.iter().fold(vec![vec![]; n], |mut adj, q| {
            let (v, u) = (q[0] as usize, q[1] as usize);
            adj[v].push(u);
            adj
        });

        let adj_rev = invocations.iter().fold(vec![vec![]; n], |mut adj, q| {
            let (v, u) = (q[0] as usize, q[1] as usize);
            adj[u].push(v);
            adj[v].push(u);
            adj
        });

        let mut seen1 = vec![false; n];
        dfs(k, &adj, &mut seen1);

        let mut seen2 = vec![false; n];
        for i in 0..n {
            if !seen2[i] && !seen1[i] {
                dfs(i, &adj_rev, &mut seen2);
            }
        }

        (0..n).filter(|&v| seen2[v]).map(|v| v as i32).collect()
    }
}
