// https://leetcode.com/problems/maximize-consecutive-elements-in-an-array-after-modification

struct Solution;

impl Solution {
    pub fn max_selected_elements(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        use std::collections::HashMap;
        let mut dp: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            let prev = dp.get(&(num - 1)).copied().unwrap_or_default();
            let x = dp.get(&num).copied().unwrap_or_default();
            let x1 = x.max(prev + 1);

            let prev = dp.get(&num).copied().unwrap_or_default();
            let x = dp.get(&(num + 1)).copied().unwrap_or_default();
            let x2 = x.max(prev + 1);

            dp.insert(num, x1);
            dp.insert(num + 1, x2);
        }

        dp.values().max().copied().unwrap_or_default()
    }
}
