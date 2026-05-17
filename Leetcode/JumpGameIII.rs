// https://leetcode.com/problems/jump-game-iii

struct Solution;

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let n = arr.len();
        let start = start as usize;

        use std::collections::VecDeque;
        let mut q = VecDeque::new();
        q.push_back(start);

        let mut seen = vec![false; n];
        seen[start] = true;

        while let Some(cur) = q.pop_front() {
            if arr[cur] == 0 {
                return true;
            }

            let d = arr[cur] as usize;
            if d <= cur && !seen[cur - d] {
                seen[cur - d] = true;
                q.push_back(cur - d);
            }
            if cur + d < n && !seen[cur + d] {
                seen[cur + d] = true;
                q.push_back(cur + d);
            }
        }

        false
    }
}
