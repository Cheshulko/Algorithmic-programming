// https://leetcode.com/problems/count-days-spent-together

struct Solution;

impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        let parse = |s: String| -> (usize, usize) {
            let (s1, s2) = s.split_once('-').unwrap();

            (s1.parse::<usize>().unwrap() - 1, s2.parse().unwrap())
        };

        let (a_start, a_end) = (parse(arrive_alice), parse(leave_alice));
        let (b_start, b_end) = (parse(arrive_bob), parse(leave_bob));

        let common = (a_start.max(b_start), a_end.min(b_end));
        if common.1 < common.0 {
            return 0;
        } else {
            if common.0 .0 == common.1 .0 {
                return (common.1 .1 - common.0 .1 + 1) as i32;
            } else {
                let mut s = months[common.0 .0] - common.0 .1 + 1 + common.1 .1;
                for i in common.0 .0 + 1..common.1 .0 {
                    s += months[i];
                }

                return s as i32;
            }
        }
    }
}
