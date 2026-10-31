// https://leetcode.com/problems/twisted-mirror-path-count

struct Solution;

impl Solution {
    pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        const M: usize = 1_000_000_000 + 7;

        const L: usize = 0;
        const D: usize = 1;

        let (n, m) = (grid.len(), grid[0].len());

        let mut dp = vec![vec![[0, 0]; m + 1]; n + 1];
        assert!(grid[0][0] == 0);
        dp[0][0][0] = 1;

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 0 {
                    dp[i][j + 1][L] += dp[i][j][L] + dp[i][j][D];
                    dp[i][j + 1][L] %= M;

                    dp[i + 1][j][D] += dp[i][j][L] + dp[i][j][D];
                    dp[i + 1][j][D] %= M;
                } else {
                    dp[i][j + 1][L] += dp[i][j][D];
                    dp[i][j + 1][L] %= M;

                    dp[i + 1][j][D] += dp[i][j][L];
                    dp[i + 1][j][D] %= M;
                }
            }
        }

        ((dp[n - 1][m - 1][L] + dp[n - 1][m - 1][D]) % M) as i32
    }
}
