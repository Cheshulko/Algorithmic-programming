// https://leetcode.com/problems/climbing-stairs-ii

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32, mut costs: Vec<i32>) -> i32 {
        let n = n as usize;

        costs.extend_from_slice(&[0, 0, 0]);

        let mut dp = vec![i32::MAX; n + 3 + 1];
        dp[0] = 0;
        for i in 0..n {
            for step in 1..=3 {
                dp[i + step] = dp[i + step].min(dp[i] + costs[i + step - 1] + (step * step) as i32);
            }
        }

        dp[n]
    }
}
