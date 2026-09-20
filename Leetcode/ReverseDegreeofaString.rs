// https://leetcode.com/problems/reverse-degree-of-a-string

struct Solution;

impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        let deg = (0..26).fold([0; 26], |mut v, x| {
            v[x] = 26 - x;
            v
        });

        s.into_bytes()
            .into_iter()
            .enumerate()
            .map(|(i, b)| deg[(b - b'a') as usize] * (i + 1))
            .sum::<usize>() as i32
    }
}
