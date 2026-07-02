// https://leetcode.com/problems/find-a-safe-walk-through-a-grid

struct Solution;

impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        use std::collections::VecDeque;

        let (n, m) = (grid.len(), grid[0].len());

        const DIRS: &[(i32, i32)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

        let mut q = VecDeque::new();
        let mut dist = vec![vec![i32::MAX; m]; n];
        dist[0][0] = grid[0][0];
        q.push_back((0, 0));

        while let Some((cur_i, cur_j)) = q.pop_front() {
            let d = dist[cur_i][cur_j];

            for (i, j) in DIRS.iter().filter_map(|(di, dj)| {
                let to_i = (cur_i as i32 + di) as usize;
                let to_j = (cur_j as i32 + dj) as usize;
                _ = grid.get(to_i)?.get(to_j)?;

                Some((to_i, to_j))
            }) {
                if dist[i][j] > d + grid[i][j] {
                    dist[i][j] = d + grid[i][j];
                    if grid[i][j] == 1 {
                        q.push_back((i, j));
                    } else {
                        q.push_front((i, j));
                    }
                }
            }
        }

        dist[n - 1][m - 1] < health
    }
}
