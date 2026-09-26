// https://leetcode.com/problems/evaluate-the-bracket-pairs-of-a-string

struct Solution;

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        use std::collections::HashMap;

        let knowledge = knowledge.into_iter().fold(HashMap::new(), |mut hm, v| {
            let Ok([k, v]) = TryInto::<[String; 2]>::try_into(v) else {
                panic!();
            };

            hm.insert(k, v);
            hm
        });

        let sv = s.chars().collect::<Vec<_>>();
        let n = s.len();
        let mut ans = String::new();
        let mut st = 0;
        let mut open = false;
        for i in 0..n {
            match sv[i] {
                '(' => {
                    st = i;
                    open = true;
                }
                ')' => {
                    open = false;
                    if let Some(v) = knowledge.get(&s[st + 1..i]) {
                        ans.extend(v.chars());
                    } else {
                        ans.push('?');
                    }
                }
                c => {
                    if !open {
                        ans.push(c);
                    }
                }
            }
        }

        ans
    }
}
