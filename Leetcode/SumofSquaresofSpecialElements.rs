// https://leetcode.com/problems/sum-of-squares-of-special-elements

struct Solution;

impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        nums.into_iter()
            .enumerate()
            .filter_map(|(i, x)| (n % (i + 1) == 0).then_some(x))
            .map(|x| x * x)
            .sum()
    }
}
