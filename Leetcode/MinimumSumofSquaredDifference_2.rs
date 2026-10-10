// https://leetcode.com/problems/minimum-sum-of-squared-difference

struct Solution;

impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        use std::collections::BTreeMap;

        let diff = nums1
            .iter()
            .zip(nums2.iter())
            .map(|(&a, &b)| (a - b).abs() as usize)
            .collect::<Vec<_>>();

        let ma = diff.iter().max().copied().unwrap_or_default();

        let mut diff = diff.into_iter().fold(vec![0; ma + 1], |mut diff, d| {
            diff[d] += 1;
            diff
        });

        let mut k = (k1 + k2) as usize;
        for d in (1..=ma).rev() {
            if diff[d] == 0 {
                continue;
            }
            let need = diff[d];
            if k >= need {
                diff[d - 1] += diff[d];
                diff[d] = 0;
                k -= need;
            } else {
                diff[d] -= k;
                diff[d - 1] += k;
                break;
            }
        }

        diff.into_iter()
            .enumerate()
            .map(|(d, cnt)| (d * d * cnt) as i64)
            .sum()
    }
}
