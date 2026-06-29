// https://leetcode.com/problems/number-of-strings-that-appear-as-substrings-in-word

struct Solution;

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        patterns.into_iter().filter(|p| word.contains(p)).count() as i32
    }
}
