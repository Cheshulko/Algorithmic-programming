// https://leetcode.com/problems/minimum-operations-to-sort-a-string

struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let n = s.len();
        let s = s.chars().collect::<Vec<_>>();

        let mut s_sorted = s.clone();
        s_sorted.sort_unstable();
        if s == s_sorted {
            return 0;
        }

        let mut ans = i32::MAX;
        for r in [[0, 1, 0], [1, 0, 1]] {
            let mut a = 0;
            let mut s = s.clone();
            for d in r {
                let mut part = s[d + 0..d + n - 1].to_vec();
                part.sort_unstable();

                if &s[d + 0..d + n - 1] != &part {
                    (&mut s[d + 0..d + n - 1]).sort_unstable();
                    a += 1;
                }
            }

            if s == s_sorted {
                ans = ans.min(a);
            }
        }

        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
