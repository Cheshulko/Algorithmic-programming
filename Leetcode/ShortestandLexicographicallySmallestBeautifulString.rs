// https://leetcode.com/problems/shortest-and-lexicographically-smallest-beautiful-string

struct Solution;

impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let mut len = usize::MAX;
        let mut ans: &[u8] = &[b'2'];

        let s = s.into_bytes();
        let n = s.len();
        for i in 0..n {
            let mut cnt = 0;
            for j in i..n {
                cnt += (s[j] == b'1') as i32;
                if cnt == k {
                    len = len.min(j - i + 1);
                }
            }
        }
        if len == usize::MAX {
            return String::new();
        }

        for i in 0..n {
            let mut cnt = 0;
            for j in i..n {
                cnt += (s[j] == b'1') as i32;
                if cnt == k && j - i + 1 == len {
                    ans = ans.min(&s[i..=j]);
                }
            }
        }

        String::from_utf8(ans.to_owned()).unwrap()
    }
}
