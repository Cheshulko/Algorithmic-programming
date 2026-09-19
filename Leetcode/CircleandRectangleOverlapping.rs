// https://leetcode.com/problems/circle-and-rectangle-overlapping

struct Solution;

impl Solution {
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        let x = x2.min(x_center).max(x1);
        let y = y2.min(y_center).max(y1);

        let d1 = x - x_center;
        let d2 = y - y_center;

        return d1 * d1 + d2 * d2 <= radius * radius;
    }
}
