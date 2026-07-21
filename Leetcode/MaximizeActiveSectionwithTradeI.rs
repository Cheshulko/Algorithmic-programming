// https://leetcode.com/problems/maximize-active-section-with-trade-i

struct Solution;

impl Solution {
    pub fn max_active_sections_after_trade(s: String) -> i32 {
        let mut s = s.chars().collect::<Vec<_>>();
        let all = s.iter().filter(|&c| *c == '1').count() as i32;

        s.push('1');

        let mut cnts = vec![];
        let mut cnt = 1;
        let mut prev = '1';
        for &c in s.iter() {
            if c == prev {
                cnt += 1;
            } else {
                cnts.push(cnt);
                cnt = 1;
                prev = c;
            }
        }
        cnts.push(cnt);

        if cnts.len() < 2 {
            return all;
        }

        let mut ans = all;
        for i in (1..cnts.len() - 2).step_by(2) {
            ans = ans.max(all + cnts[i] + cnts[i + 2]);
        }

        ans
    }
}
