// https://leetcode.com/problems/minimum-moves-to-equal-array-elements-ii

struct Solution;

impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mid = nums[nums.len() / 2];
        nums.into_iter().map(|x| (x - mid).abs()).sum::<i32>()
    }
}
