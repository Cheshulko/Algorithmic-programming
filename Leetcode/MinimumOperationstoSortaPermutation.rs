// https://leetcode.com/problems/minimum-operations-to-sort-a-permutation

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut up = 0;
        let mut down = 0;

        for w in nums.windows(2) {
            let &[a, b] = w else { panic!() };
            if a < b {
                up += 1;
            } else {
                down += 1;
            }
        }
        if up > 1 && down > 1 {
            return -1;
        }
        if down == 0 {
            return 0;
        }
        if up == 0 {
            return 1;
        }

        let solve_up = || {
            let n = nums.len();
            let mut ind = 0;
            for (i, w) in nums.windows(2).enumerate() {
                let &[a, b] = w else { panic!() };
                if a < b {
                    ind = i;
                } else {
                    if a - b > 1 {
                        return i32::MAX;
                    }
                }
            }
            let c1 = (ind + 1) + 1;
            let c2 = n - ind;

            return c1.min(c2) as i32;
        };

        let solve_down = || {
            let n = nums.len();
            let mut ind = 0;
            for (i, w) in nums.windows(2).enumerate() {
                let &[a, b] = w else { panic!() };
                if a > b {
                    ind = i;
                } else {
                    if b - a > 1 {
                        return i32::MAX;
                    }
                }
            }
            let c1 = ind + 1;
            let c2 = 1 + (n - ind);

            return c1.min(c2) as i32;
        };

        let ans = if up == 1 && down == 1 {
            solve_up().min(solve_down())
        } else if up == 1 {
            solve_up()
        } else {
            solve_down()
        };

        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
