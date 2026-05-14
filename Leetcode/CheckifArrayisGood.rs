// https://leetcode.com/problems/check-if-array-is-good

struct Solution;

impl Solution {
    pub fn is_good(mut nums: Vec<i32>) -> bool {
        nums.sort_unstable();

        let n = nums.len() as i32 - 1;

        nums == (1..=n).chain(std::iter::once(n)).collect::<Vec<_>>()
    }
}
