// https://leetcode.com/problems/number-of-unique-xor-triplets-i

struct Solution;

impl Solution {
    pub fn unique_xor_triplets(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len();

        let ma = nums.last().copied().unwrap();
        let mut cnt = 1;
        while cnt < ma {
            cnt <<= 1;
        }
        if ma == cnt && n > 2 {
            cnt <<= 1;
        }

        cnt
    }
}
