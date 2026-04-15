// https://leetcode.com/problems/shortest-distance-to-target-string-in-a-circular-array

struct Solution;

impl Solution {
    pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let n = words.len();
        let start_index = start_index as usize;

        words
            .into_iter()
            .enumerate()
            .filter_map(|(i, w)| (w == target).then_some(i))
            .map(|i| {
                [
                    i.abs_diff(start_index) as i32,
                    (n - i.abs_diff(start_index)) as i32,
                ]
            })
            .flatten()
            .min()
            .unwrap_or(-1)
    }
}
