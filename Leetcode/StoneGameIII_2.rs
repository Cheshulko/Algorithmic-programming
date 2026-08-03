// https://leetcode.com/problems/stone-game-iii

struct Solution;

impl Solution {
    pub fn stone_game_iii(mut stone_value: Vec<i32>) -> String {
        let n = stone_value.len();

        stone_value.push(0);
        stone_value.push(0);

        let mut dp = vec![[0; 2]; n + 3];
        for i in (0..n).rev() {
            let take_1 = stone_value[i] as i64;
            dp[i][0] = take_1 + dp[i + 1][1];
            dp[i][1] = -take_1 + dp[i + 1][0];

            let take_2 = take_1 + stone_value[i + 1] as i64;
            dp[i][0] = dp[i][0].max(take_2 + dp[i + 2][1]);
            dp[i][1] = dp[i][1].min(-take_2 + dp[i + 2][0]);

            let take_3 = take_2 + stone_value[i + 2] as i64;
            dp[i][0] = dp[i][0].max(take_3 + dp[i + 3][1]);
            dp[i][1] = dp[i][1].min(-take_3 + dp[i + 3][0]);
        }

        match dp[0][0] {
            ..=-1 => "Bob",
            0 => "Tie",
            1.. => "Alice",
        }
        .into()
    }
}
