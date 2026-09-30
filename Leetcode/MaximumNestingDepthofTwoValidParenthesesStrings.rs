// https://leetcode.com/problems/maximum-nesting-depth-of-two-valid-parentheses-strings

struct Solution;

impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let n = seq.len();
        let seq = seq.chars();

        let mut a = 0;
        let mut b = 0;
        let mut ans = vec![0; n];
        for (i, c) in seq.enumerate() {
            if c == '(' {
                if a <= b {
                    a += 1;
                    ans[i] = 0;
                } else {
                    b += 1;
                    ans[i] = 1;
                }
            } else {
                if a > b {
                    a -= 1;
                    ans[i] = 0;
                } else {
                    b -= 1;
                    ans[i] = 1;
                }
            }
        }

        ans
    }
}
