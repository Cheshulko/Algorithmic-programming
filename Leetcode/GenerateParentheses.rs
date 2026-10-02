// https://leetcode.com/problems/generate-parentheses

struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        use std::collections::HashSet;

        let n = n as usize;

        let mut w = vec![];
        w.push(HashSet::from([String::new()]));

        for i in 1..=n {
            let mut v = HashSet::new();

            for q in w[i - 1].iter() {
                v.insert(format!("({})", q));
            }

            for j in 1..=i - 1 {
                for q in w[j].iter() {
                    for p in w[i - j].iter() {
                        v.insert(format!("{}{}", q, p));
                        v.insert(format!("{}{}", p, q));
                    }
                }
            }

            w.push(v);
        }

        w[n].clone().into_iter().collect()
    }
}
