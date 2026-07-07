// https://leetcode.com/problems/concatenate-non-zero-digits-and-multiply-by-sum-i

struct Solution;

impl Solution {
    pub fn sum_and_multiply(n: i32) -> i64 {
        let (n, s) = {
            let n = format!("{}", n);
            let mut nn = 0;
            let mut s = 0;
            for c in n.chars() {
                if c != '0' {
                    nn *= 10;
                    let d = (c as u8 - b'0') as i64;
                    nn += d;
                    s += d;
                }
            }

            (nn, s)
        };

        n * s
    }
}
