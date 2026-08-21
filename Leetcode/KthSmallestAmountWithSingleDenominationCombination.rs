// https://leetcode.com/problems/kth-smallest-amount-with-single-denomination-combination

struct Solution;

impl Solution {
    pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
        pub fn lcm(a: i64, b: i64) -> i64 {
            a / gcd(a, b) * b
        }

        pub fn gcd(mut a: i64, mut b: i64) -> i64 {
            use std::mem::swap;
            assert!(a > 0 && b > 0);
            while a != 0 {
                if a < b {
                    swap(&mut a, &mut b);
                }
                a %= b;
            }
            b
        }

        let coins = coins.into_iter().map(i64::from).collect::<Vec<_>>();
        let ma = coins.iter().max().copied().unwrap() as i64;
        let n = coins.len();
        let k = k as i64;
        let mut l = 0;
        let mut r = n as i64 * ma * k;
        // (l .. r]
        while r - l > 1 {
            let m = (r + l) / 2;
            let mut all = 0;
            for mask in 1..(1 << n) {
                let mut l = 1;
                let mut cnt = 0;
                for b in 0..n {
                    if (mask & (1 << b)) > 0 {
                        cnt += 1;
                        l = lcm(l, coins[b]);
                    }
                }
                assert!(l > 0);
                let c = m / l;
                if cnt % 2 == 1 {
                    all += c;
                } else {
                    all -= c;
                }
            }
            if all < k {
                l = m;
            } else {
                r = m;
            }
        }
        r
    }
}
