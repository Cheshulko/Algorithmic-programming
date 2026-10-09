// https://leetcode.com/problems/minimum-insertions-to-balance-a-parentheses-string

struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut sum = 0;
        let mut ans = 0;
        let mut closed = false;

        for c in s.chars() {
            if c == '(' {
                if closed {
                    closed = false;
                    ans += 1;
                    if sum > 0 {
                        sum -= 1;
                    } else {
                        ans += 1;
                    }
                }

                sum += 1;
            } else {
                if closed {
                    closed = false;
                    if sum > 0 {
                        sum -= 1;
                    } else {
                        ans += 1;
                    }
                } else {
                    closed = true;
                }
            }
        }

        if closed {
            ans += 1;
            if sum > 0 {
                sum -= 1;
            } else {
                ans += 1;
            }
        }

        ans + sum * 2
    }
}
