// https://leetcode.com/problems/find-the-distance-value-between-two-arrays

struct Solution;

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut ans = 0;

        for x in arr1.into_iter() {
            if !arr2.iter().any(|&y| y.abs_diff(x) <= d as u32) {
                ans += 1
            }
        }

        ans
    }
}
