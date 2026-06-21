// https://leetcode.com/problems/maximum-ice-cream-bars

struct Solution;

impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, coins: i32) -> i32 {
        costs.sort_unstable();

        costs
            .into_iter()
            .scan(0_i64, |s, c| {
                *s += c as i64;
                Some(*s)
            })
            .take_while(|&s| s <= coins as i64)
            .count() as i32
    }
}
