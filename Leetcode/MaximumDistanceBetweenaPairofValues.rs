// https://leetcode.com/problems/maximum-distance-between-a-pair-of-values

struct Solution;

impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let m = nums2.len();

        let mut ans = usize::MIN;
        for j in 0..m {
            let s = &nums1[0..(j + 1).min(n)];
            let p = s.partition_point(|&x| x > nums2[j]);
            if p != s.len() {
                ans = ans.max(j - p);
            }
        }

        ans as i32
    }
}
