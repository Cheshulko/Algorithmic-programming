// https://leetcode.com/problems/find-the-maximum-number-of-elements-in-subset

struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut freq: HashMap<i64, i32> = HashMap::new();
        for &num in nums.iter() {
            *freq.entry(num as i64).or_default() += 1;
        }

        let mut ans = 1;
        for num in nums {
            let mut cur = num as i64;
            let mut cnt = 1;
            while let Some(&n) = freq.get(&cur) {
                if cur == 1 {
                    ans = ans.max((n - 1) / 2 * 2 + 1);
                    break;
                }
                ans = ans.max(cnt);
                if n > 1 {
                    cnt += 2;
                    cur = cur * cur;
                } else {
                    break;
                }
            }
        }

        ans
    }
}
