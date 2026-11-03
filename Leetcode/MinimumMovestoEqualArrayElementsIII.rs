// https://leetcode.com/problems/minimum-moves-to-equal-array-elements-iii

struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let ma = nums.iter().max().copied().unwrap();

        nums.into_iter().map(|x| ma - x).sum::<i32>()
    }
}
