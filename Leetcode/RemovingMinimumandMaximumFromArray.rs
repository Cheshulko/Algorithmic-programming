// https://leetcode.com/problems/removing-minimum-and-maximum-from-array

struct Solution;

impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let mi = nums.iter().min().unwrap();
        let ma = nums.iter().max().unwrap();

        let pmi = nums.iter().position(|x| x == mi).unwrap();
        let pma = nums.iter().position(|x| x == ma).unwrap();

        let n = nums.len();

        (1 + pmi.max(pma))
            .min(n - pmi.min(pma))
            .min(1 + pmi.min(pma) + n - pmi.max(pma)) as i32
    }
}
