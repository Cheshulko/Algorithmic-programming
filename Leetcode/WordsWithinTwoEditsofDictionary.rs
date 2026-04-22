// https://leetcode.com/problems/words-within-two-edits-of-dictionary

struct Solution;

impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        let dictionary = dictionary
            .into_iter()
            .map(|s| s.into_bytes())
            .collect::<Vec<_>>();

        queries
            .into_iter()
            .filter(|q| {
                let q = q.as_bytes();

                dictionary
                    .iter()
                    .map(|d| d.iter().zip(q.iter()).filter(|(a, b)| a != b).count())
                    .any(|c| c <= 2)
            })
            .collect()
    }
}
