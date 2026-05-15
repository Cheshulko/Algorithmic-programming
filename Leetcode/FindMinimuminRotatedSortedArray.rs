// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array

struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        use std::cmp::Ordering;

        let n = nums.len();

        let mut l = 0;
        let mut r = n - 1;
        while r - l > 1 {
            let m = (r + l) >> 1;
            match (nums[l].cmp(&nums[m]), nums[m].cmp(&nums[r])) {
                (Ordering::Less, Ordering::Greater) => l = m,
                (Ordering::Greater, Ordering::Less) => r = m,
                (Ordering::Greater, Ordering::Greater) => return nums[r],
                (Ordering::Less, Ordering::Less) => return nums[l],
                _ => unreachable!(),
            }
        }

        if r - l == 0 {
            return nums[l];
        }
        if r - l == 1 {
            return nums[l].min(nums[r]);
        }

        unreachable!()
    }
}
