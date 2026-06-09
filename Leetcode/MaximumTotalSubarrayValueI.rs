// https://leetcode.com/problems/maximum-total-subarray-value-i

struct Solution;

impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let ma = nums.iter().max().copied().unwrap() as i64;
        let mi = nums.iter().min().copied().unwrap() as i64;

        let k = k as i64;

        (ma - mi) * k
    }
}
