// https://leetcode.com/problems/separate-the-digits-in-an-array

struct Solution;

impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter()
            .rev()
            .fold(vec![], |mut v, mut n| {
                while n > 0 {
                    v.push(n % 10);
                    n /= 10;
                }
                v
            })
            .into_iter()
            .rev()
            .collect()
    }
}
