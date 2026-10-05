// https://leetcode.com/problems/score-of-parentheses

struct Solution;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut cnt = 0;
        let mut ans = 0;
        let mut prev = '#';

        for c in s.chars() {
            if c == '(' {
                cnt += 1;
            } else {
                if prev == '(' {
                    ans += 1 << (cnt - 1);
                }
                cnt -= 1;
            }

            prev = c;
        }

        ans
    }
}
