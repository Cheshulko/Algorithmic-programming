// https://leetcode.com/problems/process-string-with-special-operations-i

struct Solution;

impl Solution {
    pub fn process_str(s: String) -> String {
        let mut result = vec![];

        for c in s.chars() {
            match c {
                '*' => _ = result.pop(),
                '#' => result.extend(result.clone().into_iter()),
                '%' => result.reverse(),
                c => result.push(c),
            }
        }

        result.into_iter().collect()
    }
}
