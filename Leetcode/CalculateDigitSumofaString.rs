// https://leetcode.com/problems/calculate-digit-sum-of-a-string

struct Solution;

impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        use std::fmt::Write;

        let k = k as usize;

        let n = s.len();
        if n <= k {
            return s;
        }

        let mut res = String::new();
        let mut sum = 0;
        for (i, b) in s.into_bytes().into_iter().enumerate() {
            if i % k == 0 && i > 0 {
                write!(&mut res, "{}", sum);
                sum = 0;
            }
            sum += (b - b'0') as usize;
        }

        write!(&mut res, "{}", sum);

        Solution::digit_sum(res, k as i32)
    }
}
