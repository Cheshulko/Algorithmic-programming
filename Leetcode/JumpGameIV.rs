// https://leetcode.com/problems/jump-game-iv

struct Solution;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut groups: HashMap<i32, Vec<_>> = HashMap::new();
        for (i, &x) in arr.iter().enumerate() {
            groups.entry(x).or_default().push(i);
        }

        let n = arr.len();

        use std::collections::VecDeque;
        let mut q = VecDeque::new();
        q.push_back(0);

        let mut seen = vec![-1; n];
        seen[0] = 0;

        while let Some(i) = q.pop_front() {
            let d = seen[i];
            assert!(d != -1);

            if i == n - 1 {
                return d;
            }

            if let Some(v) = groups.remove(&arr[i]) {
                for to_i in v {
                    if to_i == i {
                        continue;
                    }

                    if seen[to_i] == -1 {
                        seen[to_i] = d + 1;
                        q.push_back(to_i);
                    }
                }
            }
            if i > 0 && seen[i - 1] == -1 {
                seen[i - 1] = d + 1;
                q.push_back(i - 1);
            }
            if i + 1 < n && seen[i + 1] == -1 {
                seen[i + 1] = d + 1;
                q.push_back(i + 1);
            }
        }

        unreachable!()
    }
}
