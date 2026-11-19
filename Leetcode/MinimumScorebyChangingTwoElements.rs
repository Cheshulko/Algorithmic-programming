// https://leetcode.com/problems/minimum-score-by-changing-two-elements

struct Solution;

impl Solution {
    pub fn minimize_sum(mut nums: Vec<i32>) -> i32 {
        if nums.len() <= 3 {
            return 0;
        }

        nums.sort_unstable();

        fn calc(nums: &[i32]) -> i32 {
            let n = nums.len();

            let mut mi = i32::MAX;
            let ma = nums[n - 1] - nums[0];

            for w in nums.windows(2) {
                mi = mi.min(w[1] - w[0]);
            }

            ma + mi
        }

        let n = nums.len();
        let mut ans = i32::MAX;
        {
            let mut nums1 = nums.clone();
            nums1[0] = nums1[2];
            nums1[1] = nums1[2];

            ans = ans.min(calc(&nums1));
        }
        {
            let mut nums1 = nums.clone();
            nums1[n - 1] = nums1[n - 3];
            nums1[n - 2] = nums1[n - 3];

            ans = ans.min(calc(&nums1));
        }
        {
            let mut nums1 = nums.clone();
            nums1[0] = nums1[1];
            nums1[n - 1] = nums1[n - 2];

            ans = ans.min(calc(&nums1));
        }

        ans
    }
}
