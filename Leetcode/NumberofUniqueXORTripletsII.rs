// https://leetcode.com/problems/number-of-unique-xor-triplets-ii

struct Solution;

impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let nums = nums.into_iter().map(|x| x as usize).collect::<Vec<_>>();
        let ma = nums.iter().max().copied().unwrap();

        let mut p = 1;
        while p <= ma {
            p <<= 1;
        }

        let mut pairs = vec![false; p + 1];
        for i in 0..n {
            for j in i..n {
                pairs[nums[i] ^ nums[j]] = true;
            }
        }

        let mut can = vec![false; p + 1];
        for x in 0..=p {
            for &y in nums.iter() {
                if x ^ y <= p && pairs[x ^ y] {
                    can[x] = true;
                }
            }
        }

        can.into_iter().filter(|&c| c).count() as i32
    }
}
