// https://leetcode.com/problems/minimum-cost-of-buying-candies-with-discount

struct Solution;

impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort_unstable();

        cost.into_iter()
            .rev()
            .enumerate()
            .fold(0, |sum, (i, c)| sum + (i % 3 != 2) as i32 * c)
    }
}
