// https://leetcode.com/problems/stone-game-ix

struct Solution;

impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        use std::cmp::Ordering;

        let stones = stones.into_iter().map(|s| s % 3).collect::<Vec<_>>();
        let freq = stones.into_iter().fold([0; 3], |mut freq, s| {
            freq[s as usize] += 1;
            freq
        });

        if freq[1] > 0 {
            // rem=1, Bob's turn
            let (c0, c1, c2) = (freq[0], freq[1] - 1, freq[2]);
            let (c1, c2) = (c1 - c1.min(c2), c2 - c1.min(c2));

            let alice_wins = match c2.cmp(&c1) {
                Ordering::Equal => false,
                Ordering::Less => c1 > 1 && (c0 % 2 == 1),
                Ordering::Greater => c0 % 2 == 0,
            };
            if alice_wins {
                return true;
            }
        }
        if freq[2] > 0 {
            // rem=2, Bob's turn
            let (c0, c1, c2) = (freq[0], freq[1], freq[2] - 1);
            let (c1, c2) = (c1 - c1.min(c2), c2 - c1.min(c2));

            let alice_wins = match c2.cmp(&c1) {
                Ordering::Equal => false,
                Ordering::Less => c0 % 2 == 0,
                Ordering::Greater => c2 > 1 && (c0 % 2 == 1),
            };
            if alice_wins {
                return true;
            }
        }

        false
    }
}
