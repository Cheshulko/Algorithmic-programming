// https://leetcode.com/problems/left-and-right-sum-differences

struct Solution;

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let s = nums.iter().sum::<i32>();

        nums.into_iter()
            .scan((0, s), |(left, right), x| {
                *right -= x;
                let result = Some((*right - *left).abs() as i32);
                *left += x;

                result
            })
            .collect()
    }
}
