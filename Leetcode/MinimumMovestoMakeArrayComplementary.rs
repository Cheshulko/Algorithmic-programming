// https://leetcode.com/problems/minimum-moves-to-make-array-complementary

struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let limit = limit as usize;
        let n = nums.len();

        let mut pairs = nums
            .iter()
            .take(n / 2)
            .zip(nums.iter().rev().take(n / 2))
            .map(|(&a, &b)| ((a + b) as usize, a.min(b) as usize, a.max(b) as usize))
            .collect::<Vec<_>>();

        pairs.sort_unstable();

        let ma = pairs.last().map(|(v, _, _)| v).copied().unwrap() as usize;
        let r = 2 * ma + 1;

        let mut pref = vec![0; r];
        let mut suf = vec![0; r];

        for &(sum, mi, ma) in pairs.iter() {
            if sum != 2 * limit {
                assert!(sum + 1 < limit + ma + 1);
                pref[sum + 1] += 1;
                pref[limit + ma + 1] += 1;
            }

            if sum != 2 {
                assert!(sum - 1 > 1 + mi - 1);
                suf[sum - 1] += 1;
                suf[1 + mi - 1] += 1;
            }
        }
        for i in 1..r {
            pref[i] += pref[i - 1];
        }
        for i in (0..r - 1).rev() {
            suf[i] += suf[i + 1];
        }

        let mut ans = usize::MAX;
        for i in 0..r {
            ans = ans.min(pref[i] + suf[i]);
        }

        ans as i32
    }
}
