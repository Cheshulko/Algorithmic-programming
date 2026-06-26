// https://leetcode.com/problems/count-subarrays-with-majority-element-ii

struct Solution;

impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i64 {
        let n = nums.len();
        let shift = n as i64;

        let mut cnt_shifted = vec![0; 2 * n + 1];
        let mut ans = 0;
        let mut ok_so_far = 0;
        let mut pref = 0;
        for num in nums {
            let delta = [-1, 1][(num == target) as usize];
            pref += delta;
            ans += (pref > 0) as i64;

            let shifted = (pref + shift) as usize;

            if delta > 0 {
                ok_so_far += cnt_shifted[shifted - 1];
            } else {
                ok_so_far -= cnt_shifted[shifted];
            }

            ans += ok_so_far;
            cnt_shifted[shifted] += 1;
        }

        ans
    }
}
