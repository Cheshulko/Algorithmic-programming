// https://leetcode.com/problems/longest-subsequence-with-non-zero-bitwise-xor

struct Solution;

impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let cur = nums.iter().fold(0, |r, &n| r ^ n);
        if cur > 0 {
            return n as i32;
        }
        if nums.iter().all(|&x| x == 0) {
            return 0;
        }

        return n as i32 - 1;
    }
}
