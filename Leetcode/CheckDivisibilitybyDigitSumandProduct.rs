// https://leetcode.com/problems/check-divisibility-by-digit-sum-and-product

struct Solution;

impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let mut m = n;
        let mut s = 0;
        let mut p = 1;
        while m > 0 {
            let d = m % 10;
            s += d;
            p *= d;
            m /= 10;
        }

        n % (s + p) == 0
    }
}
