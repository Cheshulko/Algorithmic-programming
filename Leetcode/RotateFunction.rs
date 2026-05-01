// https://leetcode.com/problems/rotate-function

struct Solution;

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let s = nums.iter().sum::<i32>();

        let mut f = 0;
        for (i, &x) in nums.iter().enumerate() {
            f += i as i32 * x;
        }

        let mut ans = f;
        for x in nums.into_iter() {
            f -= s - x;
            f += (n - 1) * x;
            ans = ans.max(f);
        }

        ans
    }
}
