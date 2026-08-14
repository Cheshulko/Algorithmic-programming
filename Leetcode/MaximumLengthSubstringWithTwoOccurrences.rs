// https://leetcode.com/problems/maximum-length-substring-with-two-occurrences

struct Solution;

impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let mut freq = [0; 26];
        let mut ans = 0;
        let mut l = 0;

        let s = s.into_bytes();
        for (r, c) in s.iter().enumerate() {
            let c = (c - b'a') as usize;
            freq[c] += 1;
            if freq[c] > 2 {
                while freq[c] > 2 {
                    freq[(s[l] - b'a') as usize] -= 1;
                    l += 1;
                }
            } else {
                ans = ans.max(r - l + 1);
            }
        }

        ans as i32
    }
}
