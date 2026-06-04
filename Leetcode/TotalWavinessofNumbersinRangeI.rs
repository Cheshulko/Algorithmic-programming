// https://leetcode.com/problems/total-waviness-of-numbers-in-range-i

struct Solution;

impl Solution {
    pub fn total_waviness(num1: i32, num2: i32) -> i32 {
        let get = |mut x: i32| -> i32 {
            let mut res = 0;
            while x >= 100 {
                let [a, b, c] = [x % 10 / 1, x % 100 / 10, x % 1000 / 100];
                if a < b && b > c {
                    res += 1;
                }
                if a > b && b < c {
                    res += 1;
                }
                x /= 10;
            }

            res
        };

        (num1..=num2).map(get).sum()
    }
}
