// https://leetcode.com/problems/maximum-subarray-with-equal-products

struct Solution;

impl Solution {
    pub fn max_length(nums: Vec<i32>) -> i32 {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            if a < b {
                std::mem::swap(&mut a, &mut b);
            }
            while b != 0 {
                a %= b;
                std::mem::swap(&mut a, &mut b);
            }

            a
        }

        let n = nums.len();
        let mut best = 1;
        for i in 0..n {
            let mut g = nums[i];
            let mut l = nums[i];
            let mut p = nums[i];
            for j in i + 1..n {
                p *= nums[j];
                g = gcd(g, nums[j]);
                l = (l * nums[j]) / gcd(l, nums[j]);

                if p == l * g {
                    best = best.max(j - i + 1);
                }
            }
        }

        best as i32
    }
}
