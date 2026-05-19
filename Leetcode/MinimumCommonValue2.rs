// https://leetcode.com/problems/minimum-common-value

struct Solution;

impl Solution {
    pub fn get_common(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        use std::cmp::Ordering;

        nums1.reverse();
        nums2.reverse();

        while let (Some(&x), Some(&y)) = (nums1.last(), nums2.last()) {
            match x.cmp(&y) {
                Ordering::Less => nums1.pop(),
                Ordering::Greater => nums2.pop(),
                Ordering::Equal => return x,
            };
        }

        -1
    }
}
