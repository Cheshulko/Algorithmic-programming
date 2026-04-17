// https://leetcode.com/problems/minimum-absolute-distance-between-mirror-pairs

struct Solution;

impl Solution {
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        fn rev(mut num: i32) -> i32 {
            let mut ans = 0;
            while num > 0 {
                ans = ans * 10 + num % 10;
                num /= 10;
            }

            ans
        }

        let revs = nums.clone().into_iter().map(rev).collect::<Vec<_>>();

        use std::collections::HashMap;
        let mut ps = HashMap::new();

        let n = nums.len();
        let mut ans = usize::MAX;
        for i in 0..n {
            if let Some(&p) = ps.get(&nums[i]) {
                ans = ans.min(i - p);
            }

            ps.insert(revs[i], i);
        }

        if ans == usize::MAX {
            -1
        } else {
            ans as i32
        }
    }
}
