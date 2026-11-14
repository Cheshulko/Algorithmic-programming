// https://leetcode.com/problems/minimum-positive-sum-subarray

struct Solution;

impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let n = nums.len();
        let (l, r) = (l as usize, r as usize);

        let mut best = i32::MAX;
        for i in 0..n {
            let mut s = 0;
            for j in i..n {
                s += nums[j];
                if j - i + 1 > r {
                    break;
                }
                if j - i + 1 >= l && s > 0 {
                    best = best.min(s);
                }
            }
        }

        if best == i32::MAX {
            -1
        } else {
            best
        }
    }
}
