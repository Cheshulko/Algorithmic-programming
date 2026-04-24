// https://leetcode.com/problems/furthest-point-from-origin

struct Solution;

impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let moves = moves
            .into_bytes()
            .into_iter()
            .map(|b| match b {
                b'L' => -1,
                b'R' => 1,
                _ => 0,
            })
            .collect::<Vec<_>>();

        let case1 = moves
            .iter()
            .map(|&b| if b == 0 { -1 } else { b })
            .sum::<i32>();

        let case2 = moves
            .iter()
            .map(|&b| if b == 0 { 1 } else { b })
            .sum::<i32>();

        case1.abs().max(case2.abs())
    }
}
