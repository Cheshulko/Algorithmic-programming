// https://leetcode.com/problems/smallest-divisible-digit-product-i

struct Solution;

impl Solution {
    pub fn smallest_number(n: i32, t: i32) -> i32 {
        let check = |mut x: i32| -> bool {
            let mut pr = 1;
            while x > 0 {
                pr *= x % 10;
                x /= 10;
            }
            pr % t == 0
        };

        (n..).filter(|&x| check(x)).take(1).next().unwrap()
    }
}
