// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii

struct Solution;

impl Solution {
    pub fn find_min<T: AsRef<[i32]>>(nums: T) -> i32 {
        let nums = nums.as_ref();
        let n = nums.len();

        let mut l = 0;
        let mut r = n - 1;

        use std::cmp::Ordering;
        while r - l > 1 {
            let m = (r + l) >> 1;
            match (nums[l].cmp(&nums[m]), nums[m].cmp(&nums[r])) {
                (Ordering::Less, Ordering::Greater) => l = m,
                (Ordering::Greater, Ordering::Less) => r = m,
                (Ordering::Greater, Ordering::Greater) => return nums[r],
                (Ordering::Less, Ordering::Less) => return nums[l],

                (Ordering::Less, Ordering::Equal) => return nums[l],
                (Ordering::Equal, Ordering::Less) => return nums[m],
                (Ordering::Equal, Ordering::Equal) => {
                    return Solution::find_min(&nums[l..=m]).min(Solution::find_min(&nums[m..=r]));
                }
                (Ordering::Equal, Ordering::Greater) => l = m,
                (Ordering::Greater, Ordering::Equal) => r = m,
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
