// https://leetcode.com/problems/maximum-product-of-two-digits

struct Solution;

impl Solution {
    pub fn max_product(mut n: i32) -> i32 {
        let mut ma = 0;
        while n > 0 {
            let x = n % 10;
            n /= 10;

            let mut m = n;
            while m > 0 {
                let y = m % 10;
                m /= 10;

                ma = ma.max(x * y);
            }
        }

        ma
    }
}
