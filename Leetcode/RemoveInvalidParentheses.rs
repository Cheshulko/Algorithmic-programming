// https://leetcode.com/problems/remove-invalid-parentheses

struct Solution;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let s = s.as_str();

        let is_valid = |mask: usize| -> (bool, usize) {
            let mut sum = 0;
            let mut removed = 0;
            let mut i = 0;
            for c in s.chars() {
                if c != '(' && c != ')' {
                    continue;
                }

                if (mask & (1 << i)) == 0 {
                    removed += 1;
                    i += 1;
                    continue;
                }

                match c {
                    '(' => sum += 1,
                    ')' => sum -= 1,
                    _ => {}
                };

                if sum < 0 {
                    return (false, removed);
                }
                i += 1;
            }

            (sum == 0, removed)
        };

        let build = |mask: usize| -> String {
            let mut res = String::new();
            let mut i = 0;
            for c in s.chars() {
                if c != '(' && c != ')' {
                    res.push(c);
                    continue;
                }

                if (mask & (1 << i)) == 0 {
                    i += 1;
                    continue;
                }

                res.push(c);
                i += 1;
            }

            res
        };

        let n = s.chars().filter(|&c| c == '(' || c == ')').count();

        let (mi, valids) = (0..(1 << n))
            .filter_map(|mask| {
                let (valid, removed) = is_valid(mask);
                if valid {
                    Some((removed, mask))
                } else {
                    None
                }
            })
            .fold((n, Vec::new()), |(mi, mut v), (removed, mask)| {
                v.push((removed, mask));
                (mi.min(removed), v)
            });

        use std::collections::HashSet;
        valids
            .into_iter()
            .filter_map(|v| (v.0 == mi).then_some(build(v.1)))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }
}
