// https://leetcode.com/problems/smallest-stable-index-i

struct Solution;

impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter()
            .enumerate()
            .position(|(i, &x)| {
                let ma = nums.iter().take(i + 1).max().copied().unwrap();
                let mi = nums.iter().skip(i).min().copied().unwrap();

                ma - mi <= k
            })
            .map(|x| x as i32)
            .unwrap_or(-1)
    }
}
