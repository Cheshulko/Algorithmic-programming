// https://leetcode.com/problems/shortest-path-with-at-most-k-consecutive-identical-characters

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
        const M: usize = 26;
        const K: usize = 50 + 1;

        fn dijkstra(
            adj: &Vec<Vec<(usize, i32)>>,
            labels: &Vec<usize>,
            s: usize,
            k: usize,
        ) -> Vec<[i32; K]> {
            let n = adj.len();

            let mut dist = vec![[i32::MAX; K]; n];
            dist[s][1] = 0;

            let mut visited = vec![[false; K]; adj.len()];
            let mut queue = BinaryHeap::new();
            queue.push(Reverse((0, 1, labels[s], s)));

            while let Some(Reverse((d, have, c, u))) = queue.pop() {
                if visited[u][have] {
                    continue;
                }

                visited[u][have] = true;
                for &(v, w) in &adj[u] {
                    let v_c = labels[v];
                    let cnt = if v_c == c { have + 1 } else { 1 };
                    if cnt > k {
                        continue;
                    }

                    if dist[v][cnt] > d + w {
                        dist[v][cnt] = d + w;
                        queue.push(Reverse((d + w, cnt, v_c, v)));
                    }
                }
            }

            dist
        }

        let n = n as usize;
        let k = k as usize;
        let adj = edges.into_iter().fold(vec![vec![]; n], |mut adj, edge| {
            let &[u, v, w] = edge.as_slice() else {
                panic!()
            };
            adj[u as usize].push((v as usize, w));
            adj
        });

        let labels = labels
            .into_bytes()
            .into_iter()
            .map(|b| (b - b'a') as usize)
            .collect::<Vec<_>>();

        let dist = dijkstra(&adj, &labels, 0, k);
        let d = dist[n - 1].iter().min().unwrap_or(&i32::MAX).clone();
        if d == i32::MAX {
            -1
        } else {
            d
        }
    }
}
