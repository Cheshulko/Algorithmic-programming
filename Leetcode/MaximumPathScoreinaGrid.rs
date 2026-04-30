// https://leetcode.com/problems/maximum-path-score-in-a-grid

struct Solution;

impl Solution {
    pub fn max_path_score(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let n = grid.len();
        let m = grid[0].len();

        let mut dp = vec![vec![vec![i32::MIN; k + 2]; m + 1]; n + 1];
        dp[0][0][(grid[0][0] > 0) as usize] = grid[0][0];
        for i in 0..n {
            for j in 0..m {
                if i + 1 < n {
                    let g = (grid[i + 1][j] > 0) as usize;
                    for v in 0..=k {
                        if dp[i][j][v] == i32::MIN {
                            continue;
                        }
                        dp[i + 1][j][v + g] = dp[i + 1][j][v + g].max(dp[i][j][v] + grid[i + 1][j]);
                    }
                }
                if j + 1 < m {
                    let g = (grid[i][j + 1] > 0) as usize;
                    for v in 0..=k {
                        if dp[i][j][v] == i32::MIN {
                            continue;
                        }
                        dp[i][j + 1][v + g] = dp[i][j + 1][v + g].max(dp[i][j][v] + grid[i][j + 1]);
                    }
                }
            }
        }

        dp[n - 1][m - 1]
            .iter()
            .take(k + 1)
            .filter_map(|&v| (v != i32::MIN).then_some(v))
            .max()
            .unwrap_or(-1)
    }
}
