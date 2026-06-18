// https://leetcode.com/problems/angle-between-hands-of-a-clock

struct Solution;

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let hour = (hour % 12) as f64;
        let minutes = minutes as f64;
        let pi2 = 2.0 * std::f64::consts::PI;

        let mut h = pi2 * hour / 12.0;
        let m = pi2 * minutes / 60.0;

        let mpart = pi2 / 12.0;
        h += minutes * mpart / 60.0;

        let a1 = (h - m).abs();
        let a2 = pi2 - a1;

        360.0 * a1.min(a2) / pi2
    }
}
