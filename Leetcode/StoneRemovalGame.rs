// https://leetcode.com/problems/stone-removal-game

struct Solution;

impl Solution {
    pub fn can_alice_win(mut n: i32) -> bool {
        let mut cur = 10;
        let mut turn = false;
        while n >= cur {
            n -= cur;
            cur -= 1;
            turn = !turn;
            assert!(cur >= 0);
        }

        turn
    }
}
