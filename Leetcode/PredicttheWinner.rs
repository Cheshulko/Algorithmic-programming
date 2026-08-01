// https://leetcode.com/problems/predict-the-winner

struct Solution;

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        fn solve(nums: &[i32], i: usize, j: usize, turn: usize, dp: &mut Vec<Vec<[i32; 2]>>) {
            if dp[i][j][turn] != -1 {
                return;
            }

            if i == j {
                dp[i][j][turn] = nums[i] * (turn as i32);

                return;
            }

            solve(nums, i + 1, j, turn ^ 1, dp);
            solve(nums, i, j - 1, turn ^ 1, dp);

            if turn == 1 {
                dp[i][j][turn] =
                    (dp[i + 1][j][turn ^ 1] + nums[i]).max(dp[i][j - 1][turn ^ 1] + nums[j]);
            } else {
                dp[i][j][turn] = dp[i + 1][j][turn ^ 1].min(dp[i][j - 1][turn ^ 1]);
            }
        }

        let n = nums.len();

        let mut dp = vec![vec![[-1; 2]; n]; n];
        solve(&nums, 0, n - 1, 1, &mut dp);

        let sum = nums.iter().sum::<i32>();
        let half = (sum + 1) / 2;

        dp[0][n - 1][1] >= half
    }
}
