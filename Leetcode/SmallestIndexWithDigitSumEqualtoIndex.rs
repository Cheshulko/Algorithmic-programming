// https://leetcode.com/problems/smallest-index-with-digit-sum-equal-to-index

struct Solution;

impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .enumerate()
            .filter_map(|(i, mut x)| {
                let mut s = 0;
                while x > 0 {
                    s += x % 10;
                    x /= 10;
                }
                (s == i as i32).then_some(i as i32)
            })
            .next()
            .unwrap_or(-1)
    }
}
