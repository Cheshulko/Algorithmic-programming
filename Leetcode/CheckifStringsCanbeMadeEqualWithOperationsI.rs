// https://leetcode.com/problems/check-if-strings-can-be-made-equal-with-operations-i

struct Solution;

impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let s1 = s1.into_bytes();
        let s2 = s2.into_bytes();

        let mut s11 = [s1[0], s1[2]];
        let mut s12 = [s1[1], s1[3]];
        let mut s21 = [s2[0], s2[2]];
        let mut s22 = [s2[1], s2[3]];

        s11.sort_unstable();
        s12.sort_unstable();
        s21.sort_unstable();
        s22.sort_unstable();

        s11 == s21 && s12 == s22
    }
}
