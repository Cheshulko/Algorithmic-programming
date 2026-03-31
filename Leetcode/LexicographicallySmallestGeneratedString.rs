// https://leetcode.com/problems/lexicographically-smallest-generated-string

struct Solution;

impl Solution {
    pub fn generate_string(str1: String, str2: String) -> String {
        use std::collections::BTreeSet;

        let s1 = str1.into_bytes();
        let s2 = str2.into_bytes();

        let n = s1.len();
        let m = s2.len();

        let mut ans = vec![b'#'; n + m - 1];

        for i in 0..n {
            if s1[i] == b'T' {
                for j in 0..m {
                    match ans[i + j] {
                        b'#' => ans[i + j] = s2[j],
                        _ if ans[i + j] == s2[j] => {}
                        _ => return String::new(),
                    }
                }
            }
        }

        let mut actual = vec![false; n];
        let mut restrictions = vec![BTreeSet::new(); n + m];

        for i in 0..n {
            if s1[i] == b'F' {
                actual[i] = true;
                for j in (0..m).rev() {
                    if ans[i + j] != b'#' && ans[i + j] != s2[j] {
                        actual[i] = false;
                    }
                }
                if !actual[i] {
                    continue;
                }

                let mut set_last = false;
                for j in (0..m).rev() {
                    if ans[i + j] == b'#' && !set_last {
                        set_last = true;
                        restrictions[i + j].insert((s2[j], i, true));
                    } else {
                        restrictions[i + j].insert((s2[j], i, false));
                    }
                }
            }
        }

        for i in 0..n + m - 1 {
            if ans[i] == b'#' {
                let mut c = b'a';
                for &(c2, j, last) in restrictions[i].iter() {
                    if !actual[j] {
                        continue;
                    }
                    if last && c == c2 {
                        c += 1;
                    }
                }
                if c > b'z' {
                    return String::new();
                }
                ans[i] = c;
                for &(c2, j, _) in restrictions[i].iter() {
                    if c2 != c {
                        actual[j] = false;
                    }
                }
            } else {
                for &(c2, j, _) in restrictions[i].iter() {
                    if c2 != ans[i] {
                        actual[j] = false;
                    }
                }
            }
        }

        for i in 0..n {
            if actual[i] {
                return String::new();
            }
        }

        String::from_utf8(ans).unwrap()
    }
}
