// https://leetcode.com/problems/minimum-distance-between-three-equal-elements-i

struct Solution;

impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut p = vec![vec![]; 101];
        for (i, num) in nums.into_iter().enumerate() {
            p[num as usize].push(i);
        }

        let mut ans = usize::MAX;
        for num in 0..=100 {
            for w in p[num].windows(3) {
                ans = ans.min(2 * (w[2] - w[0]))
            }
        }

        if ans == usize::MAX {
            -1
        } else {
            ans as i32
        }
    }
}
