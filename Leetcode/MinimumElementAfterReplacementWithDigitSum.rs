// https://leetcode.com/problems/minimum-element-after-replacement-with-digit-sum

struct Solution;

impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        let sum = |mut x: i32| -> i32 {
            let mut s = 0;
            while x > 0 {
                s += x % 10;
                x /= 10;
            }
            s
        };

        nums.into_iter().map(sum).min().unwrap()
    }
}
