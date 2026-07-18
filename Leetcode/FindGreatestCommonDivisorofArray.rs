// https://leetcode.com/problems/find-greatest-common-divisor-of-array

struct Solution;

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let mut mi = nums.iter().min().copied().unwrap();
        let mut ma = nums.iter().max().copied().unwrap();

        while mi != 0 {
            ma %= mi;
            std::mem::swap(&mut mi, &mut ma);
        }

        ma
    }
}
