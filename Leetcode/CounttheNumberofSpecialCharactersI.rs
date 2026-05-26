// https://leetcode.com/problems/count-the-number-of-special-characters-i

struct Solution;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut seen = [false; 26];
        let mut count = [false; 26];

        let mut ans = 0;
        for c in word.chars() {
            if c.is_lowercase() {
                let c = (c as u8 - b'a') as usize;
                seen[c] = true;
            }
        }
        for c in word.chars() {
            if c.is_uppercase() {
                let c = (c as u8 - b'A') as usize;
                if seen[c] && !count[c] {
                    ans += 1;
                    count[c] = true;
                }
            }
        }

        ans
    }
}
