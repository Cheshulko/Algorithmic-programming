// https://leetcode.com/problems/minimum-operations-to-reduce-x-to-zero

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let n = nums.len();
        let x = x as i64;

        let mut suf = vec![0; n + 1];
        for i in (0..n).rev() {
            suf[i] = suf[i + 1] + nums[i] as i64;
        }

        suf.reverse();

        let mut ans = usize::MAX;
        let mut s = 0;
        for i in 0..n {
            let p = suf.partition_point(|&u| u < x - s);
            if p != suf.len() && suf[p] == x - s && i <= n - p {
                ans = ans.min(i + p);
            }
            s += nums[i] as i64;
        }

        if ans == usize::MAX {
            -1
        } else {
            ans as i32
        }
    }
}
