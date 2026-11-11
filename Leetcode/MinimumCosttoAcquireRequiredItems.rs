// https://leetcode.com/problems/minimum-cost-to-acquire-required-items

struct Solution;

impl Solution {
    pub fn minimum_cost(cost1: i32, cost2: i32, cost_both: i32, need1: i32, need2: i32) -> i64 {
        let (c1, c2, cb, n1, n2) = (
            cost1 as i64,
            cost2 as i64,
            cost_both as i64,
            need1 as i64,
            need2 as i64,
        );

        (c1 * n1 + c2 * n2)
            .min({
                let nb = n1.min(n2);
                (cb * nb) + (n1 - nb) * c1 + (n2 - nb) * c2
            })
            .min({
                let nb = n1.max(n2);
                cb * nb
            })
    }
}
