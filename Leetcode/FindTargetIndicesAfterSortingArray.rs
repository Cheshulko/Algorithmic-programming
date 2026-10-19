// https://leetcode.com/problems/find-target-indices-after-sorting-array

struct Solution;

impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let fewer = nums.iter().filter(|&&x| x < target).count() as i32;
        let equal = nums.iter().filter(|&&x| x == target).count() as i32;

        (fewer..fewer + equal).collect()
    }
}
