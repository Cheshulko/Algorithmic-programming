// https://leetcode.com/problems/maximum-alternating-sum-of-squares

struct Solution;

impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let mut nums = nums
            .into_iter()
            .map(|x| {
                let x = x as i64;
                x * x
            })
            .collect::<Vec<_>>();

        nums.sort_unstable();

        let n = nums.len();
        nums.into_iter()
            .enumerate()
            .fold(0, |s, (i, x)| if i < n / 2 { s - x } else { s + x })
    }
}
