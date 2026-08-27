// https://leetcode.com/problems/lexicographically-smallest-permutation-greater-than-target

struct Solution;

impl Solution {
    pub fn lex_greater_permutation(s: String, target: String) -> String {
        let n = s.len();

        let s = s.into_bytes().into_iter().collect::<Vec<_>>();
        let t = target.into_bytes().into_iter().collect::<Vec<_>>();

        let mut freq = s.into_iter().fold([0; 26], |mut f, c| {
            f[(c - b'a') as usize] += 1;
            f
        });

        let max_left = |freq: &[i32; 26]| -> Option<u8> {
            freq.iter()
                .enumerate()
                .filter_map(|(i, cnt)| (*cnt > 0).then_some(i as u8 + b'a'))
                .max()
        };

        let mut freq_2 = freq.clone();
        let mut right = None;
        for i in 0..n {
            if let Some(ma) = max_left(&freq_2) {
                if ma > t[i] {
                    right = Some(i);
                }
            }
            let c = (t[i] - b'a') as usize;
            if freq_2[c] > 0 {
                freq_2[c] -= 1;
            } else {
                break;
            }
        }

        let Some(right) = right else {
            return String::new();
        };

        let mut ans = vec![b'#'; n];
        for i in 0..right {
            ans[i] = t[i];
            freq[(t[i] - b'a') as usize] -= 1;
        }
        let mut ma = b'#';
        for c in (0..26).rev() {
            if freq[c as usize] > 0 && c + b'a' > t[right] {
                ma = c + b'a';
            }
        }

        ans[right] = ma;
        freq[(ma - b'a') as usize] -= 1;
        let mut i = right + 1;
        for c in 0..26 {
            while freq[c] > 0 {
                ans[i] = c as u8 + b'a';
                i += 1;
                freq[c] -= 1;
            }
        }

        String::from_utf8(ans).unwrap()
    }
}
