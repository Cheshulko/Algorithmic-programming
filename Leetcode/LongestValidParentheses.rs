// https://leetcode.com/problems/longest-valid-parentheses

struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        const M: usize = 3 * 10_000 + 1;

        let get_ind = |sum: i32| -> usize { (M as i32 + sum) as usize };

        let mut ps = vec![vec![]; 2 * M];
        ps[get_ind(0)].push(0);

        let mut sum = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                sum += 1;
            } else {
                sum -= 1;
            }
            ps[get_ind(sum)].push(i + 1);
        }

        for p in ps.iter_mut() {
            p.reverse();
        }

        let mut ans = 0;
        let mut sum = 0;
        for (i, c) in s.chars().enumerate() {
            let i = i + 1;

            assert!(ps[get_ind(sum)].pop().is_some());

            if let Some(&first_bad) = ps[get_ind(sum - 1)].last() {
                if first_bad > i {
                    let last_good = first_bad - 1;
                    ans = ans.max(last_good - i + 1);
                }
            } else {
                if let Some(&last_good) = ps[get_ind(sum)].first() {
                    ans = ans.max(last_good - i + 1);
                }
            }

            if c == '(' {
                sum += 1;
            } else {
                sum -= 1;
            }
        }

        ans as i32
    }
}
