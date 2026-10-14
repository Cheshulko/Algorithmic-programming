// https://leetcode.com/problems/maximize-sum-of-squares-of-digits

struct Solution;

impl Solution {
    pub fn max_sum_of_squares(num: i32, mut sum: i32) -> String {
        let mut ans = String::new();

        for _ in 0..num {
            let d = sum.min(9);
            sum -= d;
            ans.push((b'0' + (d as u8)) as char);
        }

        if sum == 0 {
            ans
        } else {
            String::new()
        }
    }
}
