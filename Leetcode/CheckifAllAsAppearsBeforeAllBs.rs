// https://leetcode.com/problems/check-if-all-as-appears-before-all-bs

struct Solution;

impl Solution {
    pub fn check_string(s: String) -> bool {
        let b = s.chars().position(|c| c == 'b').unwrap_or(usize::MAX);

        s.chars()
            .enumerate()
            .filter_map(|(i, c)| (c == 'a').then_some(i))
            .all(|i| i <= b)
    }
}
