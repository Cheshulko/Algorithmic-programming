// https://leetcode.com/problems/maximum-amount-of-money-robot-can-earn

struct Solution;

impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        let n = coins.len();
        let m = coins[0].len();

        let mut dp = vec![vec![[i32::MIN; 3]; m + 1]; n + 1];
        dp[1][1][2] = coins[0][0];
        dp[1][1][1] = 0;
        dp[1][1][0] = i32::MIN / 2;

        for i in 0..=n {
            for p in 0..3 {
                dp[i][0][p] = i32::MIN / 2;
            }
        }
        for j in 0..=m {
            for p in 0..3 {
                dp[0][j][p] = i32::MIN / 2;
            }
        }

        for i in 1..=n {
            for j in 1..=m {
                let c = coins[i - 1][j - 1];
                dp[i][j][0] = dp[i][j][0].max(dp[i][j - 1][0] + c);
                dp[i][j][0] = dp[i][j][0].max(dp[i - 1][j][0] + c);

                for p in 1..3 {
                    dp[i][j][p] = dp[i][j][p].max(dp[i][j - 1][p] + c);
                    dp[i][j][p] = dp[i][j][p].max(dp[i - 1][j][p] + c);

                    dp[i][j][p - 1] = dp[i][j][p - 1].max(dp[i][j - 1][p]);
                    dp[i][j][p - 1] = dp[i][j][p - 1].max(dp[i - 1][j][p]);
                }
            }
        }

        *dp[n][m].iter().max().unwrap()
    }
}
