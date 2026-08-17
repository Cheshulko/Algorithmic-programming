// https://leetcode.com/problems/stone-game-v

struct Solution;

impl Solution {
    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        use std::cmp::Ordering;

        let n = stone_value.len();

        let mut pref = vec![0; n + 1];
        for i in 0..n {
            pref[i + 1] = pref[i] + stone_value[i];
        }

        let mut dp = vec![vec![0; n]; n];
        for size in 2..=n {
            for from in 0..n + 1 - size {
                let to = from + size - 1;

                for left in from..to {
                    let sum_left = pref[left + 1] - pref[from];
                    let sum_right = pref[to + 1] - pref[left + 1];

                    match sum_left.cmp(&sum_right) {
                        Ordering::Equal => {
                            dp[from][to] = dp[from][to].max(sum_left + dp[from][left]);
                            dp[from][to] = dp[from][to].max(sum_right + dp[left + 1][to]);
                        }
                        Ordering::Less => {
                            dp[from][to] = dp[from][to].max(sum_left + dp[from][left])
                        }
                        Ordering::Greater => {
                            dp[from][to] = dp[from][to].max(sum_right + dp[left + 1][to])
                        }
                    }
                }
            }
        }

        dp[0][n - 1]
    }
}
