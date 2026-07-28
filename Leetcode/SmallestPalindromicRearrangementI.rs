// https://leetcode.com/problems/smallest-palindromic-rearrangement-i

struct Solution;

impl Solution {
    pub fn smallest_palindrome(s: String) -> String {
        let n = s.len();

        let freq = s.into_bytes().into_iter().fold(vec![0; 26], |mut f, c| {
            f[(c - b'a') as usize] += 1;
            f
        });

        let mut ans = vec![b'#'; n];
        let mut d = 0;
        for c in 0..26 {
            for _ in 0..freq[c] / 2 {
                ans[d] = c as u8 + b'a';
                ans[n - 1 - d] = c as u8 + b'a';
                d += 1;
            }
            if freq[c] % 2 == 1 {
                ans[n / 2] = c as u8 + b'a';
            }
        }

        String::from_utf8(ans).unwrap()
    }
}
