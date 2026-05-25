// https://leetcode.com/problems/jump-game-vii

struct Solution;

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let (mi, ma) = (min_jump as usize, max_jump as usize);

        let s = s
            .into_bytes()
            .into_iter()
            .map(|b| b'0' == b)
            .collect::<Vec<_>>();

        let n = s.len();

        let mut pref = vec![0; n + 1];
        let mut can = vec![false; n + 1];
        can[0] = true;
        pref[1] = 1;

        for i in 2..mi {
            pref[i] += pref[i - 1];
        }

        for i in mi..n {
            pref[i] += pref[i - 1];

            let left = i.saturating_sub(ma);
            let right = 1 + i - mi;
            can[i] = s[i] && pref[right] - pref[left] > 0;
            pref[i + 1] += can[i] as i32;
        }

        can[n - 1]
    }
}
