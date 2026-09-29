// https://leetcode.com/problems/check-if-there-is-a-valid-parentheses-string-path

struct Solution;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        const MAX: usize = 200;

        let (n, m) = (grid.len(), grid[0].len());

        let mut dp = vec![vec![[false; MAX + 1]; m + 1]; n + 1];
        dp[1][0][0] = true;
        dp[0][1][0] = true;

        for i in 1..=n {
            for j in 1..=m {
                if grid[i - 1][j - 1] == '(' {
                    for open in 0..MAX {
                        dp[i][j][open + 1] |= dp[i - 1][j][open];
                        dp[i][j][open + 1] |= dp[i][j - 1][open];
                    }
                } else {
                    for open in 1..=MAX {
                        dp[i][j][open - 1] |= dp[i - 1][j][open];
                        dp[i][j][open - 1] |= dp[i][j - 1][open];
                    }
                }
            }
        }

        dp[n][m][0]
    }
}
