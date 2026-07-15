// https://leetcode.com/problems/gcd-of-odd-and-even-sums

struct Solution;

impl Solution {
    pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
        pub fn gcd(mut a: usize, mut b: usize) -> usize {
            use std::mem::swap;

            if a == 0 {
                return b;
            }
            if b == 0 {
                return a;
            }

            while a != 0 {
                if a < b {
                    swap(&mut a, &mut b);
                }
                a %= b;
            }
            b
        }

        let n = n as usize;

        gcd(
            (1..).step_by(2).take(n).sum(),
            (2..).step_by(2).take(n).sum(),
        ) as i32
    }
}
