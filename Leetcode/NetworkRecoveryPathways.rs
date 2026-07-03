// https://leetcode.com/problems/network-recovery-pathways

struct Solution;

impl Solution {
    pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
        fn dijkstra(adj: &Vec<Vec<(usize, i64)>>, online: &Vec<bool>, mi: i64) -> Vec<i64> {
            use std::cmp::Reverse;
            use std::collections::BinaryHeap;

            let s = 0;
            assert!(online[s]);

            let mut dist = vec![i64::MAX; adj.len()];
            dist[s] = 0;

            let mut visited = vec![false; adj.len()];
            let mut queue = BinaryHeap::new();

            queue.push(Reverse((0, s)));
            while let Some(Reverse((d, u))) = queue.pop() {
                if visited[u] {
                    continue;
                }

                visited[u] = true;
                for &(v, w) in &adj[u] {
                    if !online[v] {
                        continue;
                    }
                    if w < mi {
                        continue;
                    }
                    if dist[v] > d + w {
                        dist[v] = d + w;
                        queue.push(Reverse((dist[v], v)));
                    }
                }
            }

            dist
        }

        let n = online.len();
        let adj = edges.into_iter().fold(vec![vec![]; n], |mut adj, e| {
            let (u, v, c) = (e[0] as usize, e[1] as usize, e[2] as i64);

            adj[u].push((v, c));
            adj
        });

        let ma = adj
            .iter()
            .map(|l| l.iter().map(|(_, c)| c).max().copied().unwrap_or(0))
            .max()
            .unwrap()
            + 1;

        let mut l = 0;
        let mut r = ma;
        while r - l > 1 {
            let m = (l + r) >> 1;
            let dist = dijkstra(&adj, &online, m);
            if dist[n - 1] <= k {
                l = m;
            } else {
                r = m;
            }
        }

        let dist = dijkstra(&adj, &online, l);
        if dist[n - 1] > k {
            -1
        } else {
            l as i32
        }
    }
}
