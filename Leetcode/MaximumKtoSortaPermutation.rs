// https://leetcode.com/problems/maximum-k-to-sort-a-permutation

struct Solution;

impl Solution {
    pub fn sort_permutation(nums: Vec<i32>) -> i32 {
        let mut k = i32::MAX;
        for (i, &x) in nums.iter().enumerate() {
            let xu = x as usize;
            if xu == i {
                continue;
            }

            k &= x & nums[xu];
        }

        if k == i32::MAX {
            0
        } else {
            k
        }
    }
}
