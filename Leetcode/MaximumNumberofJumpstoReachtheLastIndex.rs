// https://leetcode.com/problems/maximum-number-of-jumps-to-reach-the-last-index

struct Solution;

impl Solution {
    pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();

        let target = target as u64;
        let nums = nums.into_iter().map(i64::from).collect::<Vec<_>>();

        let mut dp = vec![-1; n + 1];
        dp[0] = 0;
        for (i, &num1) in nums.iter().enumerate() {
            if dp[i] == -1 {
                continue;
            }
            for (j, &num2) in nums.iter().enumerate().skip(i + 1) {
                if num2.abs_diff(num1) <= target {
                    dp[j] = dp[j].max(dp[i] + 1);
                }
            }
        }

        dp[n - 1]
    }
}
