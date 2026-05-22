// https://leetcode.com/problems/search-in-rotated-sorted-array

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;

        let n = nums.len();

        let mut l = 0;
        let mut r = n - 1;
        while r - l > 1 {
            let m = (r + l) >> 1;
            match nums[l].cmp(&nums[m]) {
                Ordering::Less if nums[l] <= target && target <= nums[m] => r = m,
                _ => match nums[m].cmp(&nums[r]) {
                    Ordering::Less if nums[m] <= target && target <= nums[r] => l = m,
                    Ordering::Less => r = m,
                    _ => l = m,
                },
            }
        }

        if nums[l] == target {
            return l as i32;
        }
        if nums[r] == target {
            return r as i32;
        }

        -1
    }
}
