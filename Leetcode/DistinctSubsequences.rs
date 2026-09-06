// https://leetcode.com/problems/distinct-subsequences

struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let n = s.len();

        let mut dp = vec![0; n];
        let mut ndp = vec![0; n];

        for (i, c1) in t.char_indices() {
            ndp.fill(0);

            let mut acc = (i == 0) as i32;
            for (j, c2) in s.char_indices() {
                if c1 == c2 {
                    ndp[j] += acc;
                }
                acc += dp[j];
            }

            std::mem::swap(&mut dp, &mut ndp);
        }

        dp.iter().sum()
    }
}
