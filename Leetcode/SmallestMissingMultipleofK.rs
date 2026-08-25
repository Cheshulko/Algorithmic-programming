// https://leetcode.com/problems/smallest-missing-multiple-of-k

struct Solution;

impl Solution {
    pub fn missing_multiple(mut nums: Vec<i32>, k: i32) -> i32 {
        use std::cmp::Ordering;

        nums.sort_unstable();
        nums.push(i32::MAX);

        let mut cur = k;
        for num in nums {
            match num.cmp(&cur) {
                Ordering::Less => {}
                Ordering::Equal => cur += k,
                Ordering::Greater => return cur,
            }
        }

        unreachable!()
    }
}
