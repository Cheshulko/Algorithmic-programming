// https://leetcode.com/problems/apply-operations-on-array-to-maximize-sum-of-squares

struct Solution;

impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = nums.len();

        const B: usize = 32;
        const M: usize = 1_000_000_000 + 7;

        let mut cnt = [0; B];
        for i in 0..n {
            for b in 0..B {
                if (nums[i] & (1 << b)) > 0 {
                    cnt[b] += 1;
                }
            }
        }

        let mut ans = 0;
        for _ in 0..k {
            let mut x = 0;

            for b in (0..B).rev() {
                if cnt[b] > 0 {
                    x |= (1 << b) as usize;
                    cnt[b] -= 1;
                }
            }

            ans = (ans + x * x) % M;
        }

        ans as i32
    }
}
