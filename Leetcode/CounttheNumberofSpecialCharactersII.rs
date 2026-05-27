// https://leetcode.com/problems/count-the-number-of-special-characters-ii

struct Solution;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut last = [-1; 26];
        let mut count = [false; 26];

        let word = word.into_bytes();
        for (i, &b) in word.iter().enumerate() {
            if b > b'Z' {
                last[(b - b'a') as usize] = i as i32;
            }
        }

        let mut ans = 0;
        for (i, b) in word.into_iter().enumerate() {
            if b <= b'Z' {
                let bb = (b - b'A') as usize;
                if !count[bb] && last[bb] != -1 && last[bb] < i as i32 {
                    ans += 1;
                }
                count[bb] = true;
            }
        }

        ans
    }
}
