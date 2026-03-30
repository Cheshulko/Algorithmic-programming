// https://leetcode.com/problems/check-if-strings-can-be-made-equal-with-operations-ii

struct Solution;

impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        let s1 = s1.into_bytes();
        let s2 = s2.into_bytes();

        fn build(v: &[u8], p: impl Fn(usize) -> bool) -> Vec<&u8> {
            v.iter()
                .enumerate()
                .filter_map(|(i, c)| (p(i)).then_some(c))
                .collect::<Vec<_>>()
        }

        let mut s11 = build(&s1, |i| (i & 1) == 1);
        let mut s12 = build(&s1, |i| (i & 1) == 0);
        let mut s21 = build(&s2, |i| (i & 1) == 1);
        let mut s22 = build(&s2, |i| (i & 1) == 0);

        s11.sort_unstable();
        s12.sort_unstable();
        s21.sort_unstable();
        s22.sort_unstable();

        s11 == s21 && s12 == s22
    }
}
