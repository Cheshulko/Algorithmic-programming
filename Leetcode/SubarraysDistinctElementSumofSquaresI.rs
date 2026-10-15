// https://leetcode.com/problems/subarrays-distinct-element-sum-of-squares-i

struct Solution;

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        const M: usize = 101;

        let n = nums.len();

        let mut ans = 0;
        for i in 0..n {
            let mut cnt = [0; M];
            let mut d = 0;
            for j in i..n {
                let x = nums[j] as usize;
                if cnt[x] == 0 {
                    d += 1;
                }
                ans += d * d;
                cnt[x] += 1;
            }
        }

        ans
    }
}
