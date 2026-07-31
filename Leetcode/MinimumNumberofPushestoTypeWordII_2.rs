// https://leetcode.com/problems/minimum-number-of-pushes-to-type-word-ii

struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut freq = [0; 26];
        for c in word.bytes() {
            let c = (c - b'a') as usize;
            freq[c] += 1;
        }

        freq.sort_unstable();

        let mut ans = 0;
        let mut cnt = 0;
        for c in freq.into_iter().rev() {
            ans += c * (1 + cnt / 8);
            cnt += 1;
        }

        ans
    }
}
