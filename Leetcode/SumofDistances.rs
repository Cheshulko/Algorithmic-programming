// https://leetcode.com/problems/sum-of-distances

struct Solution;

impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        use std::collections::HashMap;

        let nums = nums.into_iter().map(|n| n as usize).collect::<Vec<_>>();

        let mut suf_cnt: HashMap<usize, usize> = HashMap::new();
        let mut suf: HashMap<usize, usize> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            *suf.entry(num).or_default() += i;
            *suf_cnt.entry(num).or_default() += 1;
        }

        let mut pref_cnt: HashMap<usize, usize> = HashMap::new();
        let mut pref: HashMap<usize, usize> = HashMap::new();
        let mut ans = vec![];
        for (i, num) in nums.into_iter().enumerate() {
            *suf.entry(num).or_default() -= i;
            *suf_cnt.entry(num).or_default() -= 1;

            let s1 = pref.get(&num).cloned().unwrap_or_default();
            let cnt1 = pref_cnt.get(&num).cloned().unwrap_or_default();
            let s2 = suf.get(&num).cloned().unwrap_or_default();
            let cnt2 = suf_cnt.get(&num).cloned().unwrap_or_default();

            ans.push((cnt1 * i - s1 + s2 - i * cnt2) as i64);

            *pref.entry(num).or_default() += i;
            *pref_cnt.entry(num).or_default() += 1;
        }

        ans
    }
}
