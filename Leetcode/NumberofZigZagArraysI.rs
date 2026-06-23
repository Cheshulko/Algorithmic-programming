// https://leetcode.com/problems/number-of-zigzag-arrays-i

struct Solution;

impl Solution {
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        const M: usize = 1_000_000_000 + 7;

        let (n, l, r) = (n as usize, l as usize, r as usize);

        let mut pref_inc = vec![0; r + 2];
        let mut pref_dec = vec![0; r + 2];
        for x in l..=r {
            pref_inc[x] = pref_inc[x - 1] + 1;
            pref_dec[x] = pref_dec[x - 1] + 1;
        }

        let mut dp_inc = vec![0; r + 1];
        let mut dp_dec = vec![0; r + 1];
        for _ in 2..=n {
            for x in l..=r {
                dp_inc[x] = pref_dec[x - 1];
                dp_dec[x] = (M + pref_inc[r] - pref_inc[x]) % M;
            }

            for x in l..=r {
                pref_inc[x] = (pref_inc[x - 1] + dp_inc[x]) % M;
                pref_dec[x] = (pref_dec[x - 1] + dp_dec[x]) % M;
            }
        }

        let mut ans = 0;
        for x in l..=r {
            ans += dp_inc[x];
            ans += dp_dec[x];
            ans %= M;
        }

        ans as i32
    }
}
