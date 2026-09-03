// https://leetcode.com/problems/construct-uniform-parity-array-ii

struct Solution;

impl Solution {
    pub fn uniform_array(nums1: Vec<i32>) -> bool {
        let has_odd = nums1.iter().any(|&x| x % 2 == 1);

        !has_odd || nums1.iter().min().copied().unwrap() % 2 == 1
    }
}
