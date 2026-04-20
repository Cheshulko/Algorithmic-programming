// https://leetcode.com/problems/two-furthest-houses-with-different-colors

struct Solution;

impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let n = colors.len();

        let mut ans = 0;
        for i in 0..n {
            for j in i + 1..n {
                if colors[i] != colors[j] {
                    ans = ans.max(j - i);
                }
            }
        }

        ans as i32
    }
}
