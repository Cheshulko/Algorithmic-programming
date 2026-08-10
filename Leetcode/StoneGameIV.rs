// https://leetcode.com/problems/stone-game-iv

struct Solution;

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let n = n as usize;

        let mut sqrs = vec![];
        for i in 1.. {
            let ii = i * i;
            if ii > n {
                break;
            }
            sqrs.push(ii);
        }

        let mut dp = vec![[false; 2]; n + 1];
        dp[0][0] = true;
        for i in 1..=n {
            dp[i][0] = true;
            for &take in sqrs.iter() {
                if take > i {
                    break;
                }

                dp[i][1] |= dp[i - take][0];
                dp[i][0] &= dp[i - take][1];
            }
        }

        dp[n][1]
    }
}
