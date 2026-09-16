// https://leetcode.com/problems/number-of-sets-of-k-non-overlapping-line-segments

struct Solution;

impl Solution {
    pub fn number_of_sets(n: i32, k: i32) -> i32 {
        const M: usize = 1_000_000_000 + 7;

        let (n, k) = (n as usize, k as usize);

        let mut dp = vec![vec![[0, 0]; k + 1]; n + 1];
        dp[0][0][0] = 1;
        for i in 1..=n - 1 {
            for j in 0..=k {
                dp[i][j][0] += dp[i - 1][j][0] + dp[i - 1][j][1];
                dp[i][j][0] %= M;

                dp[i][j][1] += dp[i - 1][j][1];
                dp[i][j][1] %= M;
            }
            for j in 1..=k {
                dp[i][j][1] += dp[i - 1][j - 1][0] + dp[i - 1][j - 1][1];
                dp[i][j][1] %= M;
            }
        }

        ((dp[n - 1][k][0] + dp[n - 1][k][1]) % M) as i32
    }
}
