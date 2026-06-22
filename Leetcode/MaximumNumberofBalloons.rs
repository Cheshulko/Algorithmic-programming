// https://leetcode.com/problems/maximum-number-of-balloons

struct Solution;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let needs = [(b'b', 1), (b'a', 1), (b'l', 2), (b'o', 2), (b'n', 1)];

        let text = text.into_bytes();

        let mut ans = usize::MAX;
        for (need, cnt) in needs.into_iter() {
            ans = ans.min(text.iter().filter(|&c| *c == need).count() / cnt);
        }

        ans as i32
    }
}
