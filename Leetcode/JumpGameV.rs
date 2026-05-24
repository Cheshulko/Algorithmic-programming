// https://leetcode.com/problems/jump-game-v

struct Solution;

impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let n = arr.len();
        let d = d as usize;

        let mut arr_sorted = (0..n).collect::<Vec<_>>();
        arr_sorted.sort_unstable_by(|&i, &j| arr[j].cmp(&arr[i]));

        let mut dp = vec![1; n];
        let mut ans = 0;

        for i in arr_sorted {
            let left = i.saturating_sub(d);
            let righ = (i + d).min(n - 1);

            let mut local_ma = arr[i];
            for j in (left..=i.saturating_sub(1)).rev() {
                if arr[i] >= arr[j] {
                    continue;
                }
                if local_ma >= arr[j] {
                    continue;
                }
                local_ma = local_ma.max(arr[j]);
                dp[i] = dp[i].max(dp[j] + 1);
            }

            let mut local_ma = arr[i];
            for j in i + 1..=righ {
                if arr[i] >= arr[j] {
                    continue;
                }
                if local_ma >= arr[j] {
                    continue;
                }
                local_ma = local_ma.max(arr[j]);
                dp[i] = dp[i].max(dp[j] + 1);
            }

            ans = ans.max(dp[i]);
        }

        ans as i32
    }
}
