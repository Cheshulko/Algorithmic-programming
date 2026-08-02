// https://leetcode.com/problems/stone-game

struct Solution;

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let n = piles.len();

        let mut dp = vec![vec![-1; n]; n];
        for i in 0..n {
            dp[i][i] = 0;
        }

        for range in 1..=n {
            for i in 0..n - range {
                let j = i + range;
                if range & 1 == 1 {
                    dp[i][j] = (dp[i + 1][j] + piles[i]).max(dp[i][j - 1] + piles[j]);
                } else {
                    dp[i][j] = (dp[i + 1][j] - piles[i]).min(dp[i][j - 1] - piles[j]);
                }
            }
        }

        dp[0][n - 1] > 0
    }
}
