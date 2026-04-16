// https://leetcode.com/problems/closest-equal-element-queries

struct Solution;

impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let M = nums.len();

        let queries = queries
            .into_iter()
            .map(|q| (q as usize, nums[q as usize]))
            .collect::<Vec<_>>();

        let mut m: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, num) in nums.into_iter().enumerate() {
            m.entry(num).or_default().push(i);
        }

        queries
            .into_iter()
            .map(|(j, n)| {
                let v = m.entry(n).or_default();
                let N = v.len();
                if N == 1 {
                    return -1;
                }
                let p1 = v.partition_point(|&i| i <= j);
                let p2 = v.partition_point(|&i| i < j);
                let v1 = v[p1 % N];
                let v2 = v[(p2 + N - 1) % N];
                let d1 = v1.abs_diff(j);
                let d2 = v2.abs_diff(j);

                d1.min(M - d1).min(d2).min(M - d2) as i32
            })
            .collect()
    }
}
