// https://leetcode.com/problems/maximum-number-of-non-overlapping-substrings

struct Solution;

impl Solution {
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        use std::collections::HashSet;

        let s = s.into_bytes().into_iter().collect::<Vec<_>>();

        let mut first = [usize::MAX; 26];
        let mut last = [usize::MIN; 26];
        let mut pos = vec![vec![]; 26];
        for (i, &c) in s.iter().enumerate() {
            let c = (c - b'a') as usize;
            first[c] = first[c].min(i);
            last[c] = last[c].max(i);
            pos[c].push(i);
        }

        let mut ranges = pos
            .into_iter()
            .filter_map(|p| {
                let l = p.len();
                if l == 0 {
                    return None;
                }

                let mut left = p[0];
                let mut right = p[l - 1];
                let mut seen = (left..=right)
                    .map(|i| (s[i] - b'a') as usize)
                    .collect::<HashSet<_>>();

                for _ in 0..26 {
                    for &c in seen.iter() {
                        left = left.min(first[c]);
                        right = right.max(last[c]);
                    }
                    let l = seen.len();
                    for j in left..=right {
                        seen.insert((s[j] - b'a') as usize);
                    }
                    if seen.len() == l {
                        break;
                    }
                }

                Some((right, left))
            })
            .collect::<Vec<_>>();

        ranges.sort_unstable();
        ranges.dedup();
        ranges.push((usize::MAX, usize::MAX));

        let mut ans = vec![];
        let mut start = ranges[0];
        let mut prev = ranges[0];
        for i in 1..ranges.len() {
            if start.0 > ranges[i].1 {
                if start.1 > ranges[i].1 {
                    continue;
                }
            } else {
                ans.push(String::from_utf8_lossy(&s[start.1..=prev.0]).to_string());
                start = ranges[i];
            }
            prev = ranges[i];
        }

        ans
    }
}
