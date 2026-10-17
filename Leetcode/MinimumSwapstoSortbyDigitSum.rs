// https://leetcode.com/problems/minimum-swaps-to-sort-by-digit-sum

struct Solution;

impl Solution {
    pub fn min_swaps(mut nums: Vec<i32>) -> i32 {
        let sum = |mut num: i32| -> i32 {
            let mut s = 0;
            while num > 0 {
                s += num % 10;
                num /= 10;
            }
            s
        };

        let n = nums.len();

        let mut nums = nums
            .into_iter()
            .enumerate()
            .map(|(i, x)| (sum(x), x, i))
            .collect::<Vec<_>>();

        nums.sort_unstable();

        let ind = nums.into_iter().map(|x| x.2).collect::<Vec<_>>();

        let mut pos = (0..n).collect::<Vec<_>>();
        let mut pos_rev = (0..n).collect::<Vec<_>>();
        let mut ans = 0;

        for i in 0..n {
            if pos[i] == ind[i] {
                continue;
            }
            ans += 1;

            let i_i = pos_rev[ind[i]];
            let xx = pos[i];
            let yy = pos[pos_rev[ind[i]]];

            pos.swap(i, i_i);
            pos_rev.swap(xx, yy);
        }

        ans
    }
}
