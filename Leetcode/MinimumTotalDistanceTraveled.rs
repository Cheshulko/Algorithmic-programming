// https://leetcode.com/problems/minimum-total-distance-traveled

struct Solution;

impl Solution {
    pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
        robot.sort_unstable();
        factory.sort_unstable();

        let m = factory.len();
        let n = robot.len();

        let mut dp = vec![vec![i64::MAX; n + 1]; m + 1];
        for i in 0..=m {
            dp[i][0] = 0;
        }
        for f in 1..=m {
            for r in 1..=n {
                dp[f][r] = dp[f][r].min(dp[f - 1][r]);
                let mut d = 0;
                for prev_r in (1..=r).rev() {
                    if (factory[f - 1][1] as usize) < r - prev_r + 1 {
                        break;
                    }
                    d += (factory[f - 1][0] - robot[prev_r - 1]).abs() as i64;

                    if dp[f - 1][prev_r - 1] != i64::MAX {
                        dp[f][r] = dp[f][r].min(dp[f - 1][prev_r - 1] + d);
                    }
                }
            }
        }

        dp[m][n]
    }
}
