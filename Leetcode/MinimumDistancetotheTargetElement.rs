// https://leetcode.com/problems/minimum-distance-to-the-target-element

struct Solution;

impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        nums.into_iter()
            .enumerate()
            .filter_map(|(i, x)| (x == target).then_some(i))
            .map(|i| i.abs_diff(start as usize))
            .min()
            .unwrap() as i32
    }
}
