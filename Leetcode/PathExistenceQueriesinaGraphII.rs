// https://leetcode.com/problems/path-existence-queries-in-a-graph-ii

struct LCA {
    up: Vec<Vec<usize>>,
    pub depth: Vec<usize>,
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

    fn get(&self, mut u: usize, mut v: usize) -> usize {
        if self.depth[u] < self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }

        // depth[u] >= depth[v]

        // 1. Lift u to the same depth as v
        let diff = self.depth[u] - self.depth[v];
        let ma = self.depth[u].max(self.depth[v]);
        for i in 0..self.max_log {
            if (diff >> i) & 1 == 1 {
                u = self.up[u][i];
            }
        }

        assert!(self.depth[u] == self.depth[v]);

        if u > v {
            return 1 + ma - self.depth[u];
        } else {
            return ma - self.depth[u];
        }
    }
}

struct Solution;

impl Solution {
    pub fn path_existence_queries(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;

        let mut nums = nums
            .into_iter()
            .enumerate()
            .map(|(i, n)| (n, i))
            .collect::<Vec<_>>();

        nums.sort_unstable();

        let mut to_ind = vec![0; n];
        for (i, &(_, j)) in nums.iter().enumerate() {
            to_ind[j] = i;
        }

        let mut group = vec![0; n];
        for i in 1..n {
            if nums[i].0 - nums[i - 1].0 <= max_diff {
                group[i] = group[i - 1];
            } else {
                group[i] = group[i - 1] + 1;
            }
        }

        let mut adj = vec![vec![]; n + 1];
        let mut i = 0;
        let mut j = 1;
        adj[n].push(0);
        adj[0].push(n);
        for i in 1..n {
            if group[i] != group[i - 1] {
                adj[n].push(i);
                adj[i].push(n);
            }
        }

        while j < n {
            if i >= j {
                j += 1;
                continue;
            }

            if nums[j].0 - nums[i].0 <= max_diff {
                adj[i].push(j);
                adj[j].push(i);
                j += 1;
            } else {
                i += 1;
            }
        }

        let lca = LCA::new(n, &adj);
        queries
            .into_iter()
            .map(|q| {
                let q0 = to_ind[q[0] as usize];
                let q1 = to_ind[q[1] as usize];
                if q0 == q1 {
                    return 0;
                }
                if nums[q0].0 == nums[q1].0 {
                    return 1;
                }

                let g0 = group[q0];
                let g1 = group[q1];
                if g0 != g1 || max_diff == 0 {
                    return -1;
                }

                lca.get(q0, q1).max(1) as i32
            })
            .collect()
    }
}
