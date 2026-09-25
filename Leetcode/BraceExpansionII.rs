// https://leetcode.com/problems/brace-expansion-ii

use std::collections::HashSet;

#[derive(Debug)]
enum Seq {
    Open,
    Close,
    Comma,
    Part(HashSet<String>),
}

struct Solution;

impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        pub fn mult(v1: HashSet<String>, v2: HashSet<String>) -> HashSet<String> {
            let mut res = HashSet::new();

            for e1 in v1.into_iter() {
                for e2 in v2.iter() {
                    let mut e = e1.clone();
                    e.push_str(e2);

                    res.insert(e);
                }
            }

            res
        }

        pub fn add(v1: HashSet<String>, v2: HashSet<String>) -> HashSet<String> {
            v1.union(&v2).into_iter().map(ToOwned::to_owned).collect()
        }

        let mut q = vec![];
        for c in expression.chars() {
            match c {
                '{' => q.push(Seq::Open),
                '}' => q.push(Seq::Close),
                ',' => q.push(Seq::Comma),
                _ => {
                    let part = HashSet::from([c.to_string()]);
                    q.push(Seq::Part(part));
                }
            }

            let mut found_close = false;
            while let Some(prev) = q.pop() {
                match prev {
                    Seq::Open => {
                        if !found_close {
                            q.push(prev);
                        }
                        break;
                    }
                    Seq::Close => found_close = true,
                    Seq::Comma => {
                        q.push(prev);
                        break;
                    }
                    Seq::Part(prev) => {
                        if let Some(prev_to_prev) = q.pop() {
                            match prev_to_prev {
                                Seq::Open => {
                                    if found_close {
                                        found_close = false;
                                        q.push(Seq::Part(prev));
                                    } else {
                                        q.push(prev_to_prev);
                                        q.push(Seq::Part(prev));
                                        break;
                                    }
                                }
                                Seq::Close => unreachable!(),
                                Seq::Comma => {
                                    if found_close {
                                        let Seq::Part(prev_to_comma) = q.pop().unwrap() else {
                                            panic!()
                                        };
                                        q.push(Seq::Part(add(prev_to_comma, prev)));
                                    } else {
                                        q.push(Seq::Comma);
                                        q.push(Seq::Part(prev));
                                        break;
                                    }
                                }
                                Seq::Part(x) => {
                                    q.push(Seq::Part(mult(x, prev)));
                                }
                            }
                        } else {
                            q.push(Seq::Part(prev));
                            break;
                        }
                    }
                }
            }
        }

        while q.len() > 1 {
            assert!(q.len() >= 3);
            let Seq::Part(x) = q.pop().unwrap() else {
                panic!()
            };
            let Seq::Comma = q.pop().unwrap() else {
                panic!()
            };
            let Seq::Part(y) = q.pop().unwrap() else {
                panic!()
            };
            q.push(Seq::Part(add(x, y)));
        }

        let Seq::Part(res) = q.pop().unwrap() else {
            panic!()
        };
        assert!(q.is_empty());

        let mut res = res.into_iter().collect::<Vec<_>>();
        res.sort_unstable();

        res
    }
}
