// https://leetcode.com/problems/smallest-stable-index-ii

struct Solution;

impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();

        let mut pref_max = vec![i32::MIN; n + 1];
        for i in 1..=n {
            pref_max[i] = pref_max[i - 1].max(nums[i - 1]);
        }

        let mut suf_min = vec![i32::MAX; n + 1];
        for i in (0..n).rev() {
            suf_min[i] = suf_min[i + 1].min(nums[i]);
        }

        nums.iter()
            .enumerate()
            .position(|(i, _)| {
                let ma = pref_max[i + 1];
                let mi = suf_min[i];

                ma - mi <= k
            })
            .map(|x| x as i32)
            .unwrap_or(-1)
    }
}
