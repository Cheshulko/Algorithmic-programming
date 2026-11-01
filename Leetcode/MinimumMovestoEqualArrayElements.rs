// https://leetcode.com/problems/minimum-moves-to-equal-array-elements

struct Solution;

impl Solution {
    pub fn min_moves(mut nums: Vec<i32>) -> i32 {
        let mi = nums.iter().min().copied().unwrap();

        nums.into_iter().map(|x| x - mi).sum::<i32>()
    }
}
