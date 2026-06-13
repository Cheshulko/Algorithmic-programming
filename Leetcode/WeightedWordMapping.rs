// https://leetcode.com/problems/weighted-word-mapping

struct Solution;

impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        words
            .into_iter()
            .map(|word| {
                let s = word
                    .into_bytes()
                    .into_iter()
                    .map(|b| weights[(b - b'a') as usize])
                    .sum::<i32>();

                (25 - (s % 26) as u8 + b'a') as char
            })
            .collect()
    }
}
