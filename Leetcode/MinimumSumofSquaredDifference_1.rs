// https://leetcode.com/problems/minimum-sum-of-squared-difference

struct Solution;

impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        use std::collections::BTreeMap;

        let mut diff: BTreeMap<i64, i64> = BTreeMap::new();
        for (&a, &b) in nums1.iter().zip(nums2.iter()) {
            *diff.entry((a - b).abs() as i64).or_default() += 1;
        }

        diff.entry(0).or_insert(0);

        let mut k = (k1 + k2) as i64;
        while k > 0 {
            if let Some((d1, cnt1)) = diff.pop_last() {
                if d1 == 0 {
                    break;
                }

                if let Some((d2, cnt2)) = diff.pop_last() {
                    let need = (d1 - d2) * cnt1;
                    if k >= need {
                        k -= need;
                        diff.insert(d2, cnt1 + cnt2);
                    } else {
                        let c = k / cnt1;
                        let r = k % cnt1;
                        // -c -c -c -c -c -c -c
                        // -1 -1 -1 -1
                        *diff.entry(d1 - c - 1).or_default() += r;
                        *diff.entry(d1 - c).or_default() += cnt1 - r;
                        *diff.entry(d2).or_default() += cnt2;
                        k = 0;
                    }
                } else {
                    unreachable!()
                }
            } else {
                break;
            }
        }

        diff.into_iter().map(|(k, v)| k * k * v).sum()
    }
}
