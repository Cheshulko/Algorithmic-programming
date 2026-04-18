// https://leetcode.com/problems/mirror-distance-of-an-integer

struct Solution;

impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        let mut ans = 0;
        let mut num = n;
        while num > 0 {
            ans = ans * 10 + num % 10;
            num /= 10;
        }

        (ans - n).abs()
    }
}
