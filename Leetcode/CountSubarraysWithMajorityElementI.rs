// https://leetcode.com/problems/count-subarrays-with-majority-element-i

struct Solution;

impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();

        let mut ans = 0;
        for l in 0..n {
            let mut c1 = 0;
            let mut c2 = 0;
            for &el in nums.iter().skip(l) {
                c1 += (el == target) as usize;
                c2 += (el != target) as usize;

                ans += (c1 > c2) as i32;
            }
        }

        ans
    }
}
