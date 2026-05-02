// https://leetcode.com/problems/rotated-digits

struct Solution;

impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        (1..=n)
            .filter(|&(mut n)| {
                let mut d = 0;
                while n > 0 {
                    if matches!(n % 10, 3 | 4 | 7) {
                        return false;
                    }
                    if matches!(n % 10, 2 | 5 | 6 | 9) {
                        d += 1;
                    }

                    n /= 10;
                }

                d > 0
            })
            .count() as i32
    }
}
