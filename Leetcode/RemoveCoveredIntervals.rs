// https://leetcode.com/problems/remove-covered-intervals

struct Solution;

impl Solution {
    pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Ordering;
        intervals.sort_unstable_by(|a, b| match a[0].cmp(&b[0]) {
            Ordering::Equal => b[1].cmp(&a[1]),
            x => x,
        });

        let (mut l, mut r) = (0, 0);
        intervals
            .into_iter()
            .filter(|int| {
                if r < int[1] {
                    l = int[0];
                    r = int[1];
                    return true;
                } else if l <= int[0] && int[1] <= r {
                    return false;
                } else {
                    return true;
                }
            })
            .count() as i32
    }
}
