// https://leetcode.com/problems/smallest-missing-integer-greater-than-sequential-prefix-sum

struct Solution;

impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let mut prev = nums[0];
        let mut sum = prev;
        for &x in nums.iter().skip(1) {
            if x == prev + 1 {
                sum += x;
                prev = x;
            } else {
                break;
            }
        }

        loop {
            let mut seen = false;
            for &x in nums.iter() {
                if x == sum {
                    seen = true;
                    break;
                }
            }

            if !seen {
                return sum;
            } else {
                sum += 1;
            }
        }
    }
}
