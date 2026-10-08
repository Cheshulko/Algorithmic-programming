// https://leetcode.com/problems/remove-outermost-parentheses

struct Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut ans = String::new();
        let mut sum = 0;
        for c in s.chars() {
            if c == '(' {
                if sum != 0 {
                    ans.push(c);
                }
                sum += 1;
            } else {
                if sum != 1 {
                    ans.push(c);
                }
                sum -= 1;
            }
        }

        ans
    }
}
