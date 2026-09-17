// https://leetcode.com/problems/find-two-non-overlapping-sub-arrays-each-with-target-sum

struct Solution;

impl Solution {
    pub fn min_sum_of_lengths(mut arr: Vec<i32>, target: i32) -> i32 {
        fn find(arr: &[i32], target: i32) -> Vec<(usize, usize)> {
            use std::collections::HashMap;

            let n = arr.len();

            let mut result = vec![];
            let mut sum = 0;
            let mut have = HashMap::new();
            for i in 0..n {
                sum += arr[i];
                if sum == target {
                    result.push((i, 0));
                }
                if let Some(&j) = have.get(&(sum - target)) {
                    result.push((i, j + 1));
                }
                have.insert(sum, i);
            }

            result
        }

        let n = arr.len();

        let mut best_pref = vec![usize::MAX; n];
        for (i, j) in find(&arr, target) {
            assert!(best_pref[i] == usize::MAX);
            best_pref[i] = i - j + 1;
        }
        for i in 1..n {
            best_pref[i] = best_pref[i].min(best_pref[i - 1]);
        }

        arr.reverse();
        let mut best_suf = vec![usize::MAX; n];
        for (i, j) in find(&arr, target) {
            let i = n - 1 - i;
            let j = n - 1 - j;
            assert!(best_suf[i] == usize::MAX);
            best_suf[i] = j - i + 1;
        }
        for i in (0..n - 1).rev() {
            best_suf[i] = best_suf[i].min(best_suf[i + 1]);
        }

        let mut ans = usize::MAX;
        for i in 0..n - 1 {
            if best_pref[i] != usize::MAX && best_suf[i + 1] != usize::MAX {
                ans = ans.min(best_pref[i] + best_suf[i + 1]);
            }
        }

        if ans == usize::MAX {
            -1
        } else {
            ans as i32
        }
    }
}
