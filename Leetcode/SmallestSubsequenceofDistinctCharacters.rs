// https://leetcode.com/problems/smallest-subsequence-of-distinct-characters

struct Solution;

impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let s = s.chars().collect::<Vec<_>>();

        let mut cnt = vec![0; 26];
        for &c in s.iter() {
            let c = (c as u8 - b'a') as usize;
            cnt[c] += 1;
        }

        let mut ans = vec![];
        let mut used = vec![false; 26];
        for c in s.into_iter() {
            let c_ = (c as u8 - b'a') as usize;
            cnt[c_] -= 1;

            if used[c_] {
                continue;
            }

            while let Some(&last) = ans.last() {
                let last_ = (last as u8 - b'a') as usize;
                if cnt[last_] == 0 {
                    break;
                } else if last > c {
                    used[last_] = false;
                    ans.pop();
                } else {
                    break;
                }
            }

            used[c_] = true;
            ans.push(c);
        }

        ans.into_iter().collect()
    }
}
