// https://leetcode.com/problems/count-islands-with-total-value-divisible-by-k

struct Solution;

impl Solution {
    pub fn count_islands(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        fn dfs((i, j): (usize, usize), grid: &Vec<Vec<i32>>, seen: &mut Vec<Vec<bool>>) -> i64 {
            seen[i][j] = true;

            if grid[i][j] == 0 {
                return 0;
            }

            let (n, m) = (grid.len() as i32, grid[0].len() as i32);
            let mut s = grid[i][j] as i64;
            for di in -1..=1_i32 {
                for dj in -1..=1 {
                    if (di + dj).abs() != 1 {
                        continue;
                    }

                    let to_i = i as i32 + di;
                    let to_j = j as i32 + dj;
                    if to_i < 0 || to_i == n || to_j < 0 || to_j == m {
                        continue;
                    }
                    let to @ (to_i, to_j) = (to_i as usize, to_j as usize);
                    if seen[to_i][to_j] {
                        continue;
                    }

                    s += dfs(to, grid, seen);
                }
            }

            s
        }

        let k = k as i64;
        let (n, m) = (grid.len(), grid[0].len());
        let mut seen = vec![vec![false; m]; n];
        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                if seen[i][j] || grid[i][j] == 0 {
                    continue;
                }
                if dfs((i, j), &grid, &mut seen) % k == 0 {
                    ans += 1;
                }
            }
        }

        ans
    }
}
