// https://leetcode.com/problems/robot-return-to-origin

struct Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        moves
            .into_bytes()
            .into_iter()
            .fold((0, 0), |(x, y), b| match b {
                b'L' => (x - 1, y),
                b'R' => (x + 1, y),
                b'U' => (x, y - 1),
                b'D' => (x, y + 1),
                _ => unreachable!(),
            })
            == (0, 0)
    }
}
