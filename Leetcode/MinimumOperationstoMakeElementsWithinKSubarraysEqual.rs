// https://leetcode.com/problems/minimum-operations-to-make-elements-within-k-subarrays-equal

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32, k: i32) -> i64 {
        use std::collections::BTreeSet;

        let nums = nums.into_iter().map(i64::from).collect::<Vec<_>>();
        let (x, k, n) = (x as usize, k as usize, nums.len());
        let mut best_for_range = vec![i64::MAX; n];

        let mut right: BTreeSet<(i64, usize)> = BTreeSet::new();
        let mut left: BTreeSet<(i64, usize)> = BTreeSet::new();
        let mut sum_left = 0;
        let mut sum_right = 0;

        for i in 0..x - 1 {
            left.insert((nums[i], i));
            sum_left += nums[i];
        }
        for i in x - 1..n {
            left.insert((nums[i], i));
            sum_left += nums[i];

            while left.last().map(|x| x.0).unwrap_or(i64::MIN)
                > right.first().map(|x| x.0).unwrap_or(i64::MAX)
            {
                let ma_from_left = left.pop_last().unwrap();
                right.insert(ma_from_left);

                sum_left -= ma_from_left.0;
                sum_right += ma_from_left.0;
            }

            while left.len() > right.len() && left.len() - right.len() > 1 {
                let ma_from_left = left.pop_last().unwrap();
                right.insert(ma_from_left);

                sum_left -= ma_from_left.0;
                sum_right += ma_from_left.0;
            }

            while right.len() > left.len() && right.len() - left.len() > 0 {
                let mi_from_right = right.pop_first().unwrap();
                left.insert(mi_from_right);

                sum_left += mi_from_right.0;
                sum_right -= mi_from_right.0;
            }

            assert!(
                left.len() == right.len()
                    || (left.len() > right.len() && left.len() - right.len() == 1)
            );

            let i = i - (x - 1);
            {
                let best = left.last().unwrap().0;
                let left = (left.len() as i64) * best - sum_left;
                let right = sum_right - (right.len() as i64) * best;
                best_for_range[i] = left + right
            }

            if left.remove(&(nums[i], i)) {
                sum_left -= nums[i];
            }
            if right.remove(&(nums[i], i)) {
                sum_right -= nums[i];
            }
        }

        let mut dp = vec![vec![i64::MAX; k + 1]; n + 1 + x];
        for i in (0..n).rev() {
            dp[i][1] = best_for_range[i].min(dp[i + 1][1]);
        }

        for cur_k in 2..=k {
            for i in (0..n).rev() {
                if best_for_range[i] == i64::MAX {
                    continue;
                }

                dp[i][cur_k] = dp[i + 1][cur_k];
                if dp[i + x][cur_k - 1] != i64::MAX {
                    assert!(best_for_range[i] != i64::MAX);
                    assert!(dp[i + x][cur_k - 1] != i64::MAX);
                    dp[i][cur_k] = dp[i][cur_k].min(best_for_range[i] + dp[i + x][cur_k - 1]);
                }
            }
        }

        dp[0][k]
    }
}
