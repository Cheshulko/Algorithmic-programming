// https://leetcode.com/problems/sum-of-gcd-of-formed-pairs

struct Solution;

impl Solution {
    pub fn gcd_sum(nums: Vec<i32>) -> i64 {
        pub fn gcd(mut a: i64, mut b: i64) -> i64 {
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

        let n = nums.len();

        let mut prefixGcd = nums
            .into_iter()
            .map(i64::from)
            .scan(i64::MIN, |ma, n| {
                *ma = (*ma).max(n);

                Some(gcd(*ma, n))
            })
            .collect::<Vec<_>>();

        prefixGcd.sort_unstable();

        prefixGcd
            .iter()
            .zip(prefixGcd.iter().rev())
            .take(n / 2)
            .map(|(&a, &b)| gcd(a, b))
            .sum()
    }
}
