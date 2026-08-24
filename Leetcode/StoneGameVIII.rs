// https://leetcode.com/problems/stone-game-viii

struct Solution;

impl Solution {
    pub fn stone_game_viii(stones: Vec<i32>) -> i32 {
        let n = stones.len();

        let mut pref = stones.iter().sum::<i32>();

        let mut alice_best = pref;
        let mut bob_worse = -pref;

        for r in (1..n - 1).rev() {
            pref -= stones[r + 1];
            let alice = pref + bob_worse;
            let bob = alice_best - pref;

            alice_best = alice_best.max(alice);
            bob_worse = bob_worse.min(bob);
        }

        alice_best
    }
}
