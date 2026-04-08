// https://leetcode.com/problems/xor-after-range-multiplication-queries-i

struct Solution;

impl Solution {
    pub fn xor_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        const M: usize = 1_000_000_000 + 7;

        let mut nums = nums.into_iter().map(|x| x as usize).collect::<Vec<_>>();

        for q in queries {
            let &[l, r, k, v] = q.as_slice() else {
                panic!()
            };
            for i in (l..=r).step_by(k as usize) {
                nums[i as usize] *= v as usize;
                nums[i as usize] %= M;
            }
        }

        nums.into_iter().fold(0, |x, v| x ^ v) as i32
    }
}
