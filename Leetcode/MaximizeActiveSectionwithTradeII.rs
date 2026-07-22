// https://leetcode.com/problems/maximize-active-section-with-trade-ii

mod cm_rmq {
    use std::{
        cmp::{max, min},
        ops::Range,
    };

    pub struct RMQ<T: Ord + Copy> {
        sparse_table: Vec<Vec<T>>,
        logs2: Vec<usize>,
        f: fn(T, T) -> T,
    }

    impl<T: Ord + Copy> RMQ<T> {
        fn new(f: fn(T, T) -> T, input: &[T]) -> RMQ<T> {
            RMQ {
                sparse_table: Self::build_sparse_table(f, input),
                logs2: vec![0]
                    .into_iter()
                    .chain((1..=input.len()).map(|x| x.ilog2() as usize))
                    .collect(),
                f: f,
            }
        }

        pub fn query(&self, range: Range<usize>) -> Option<T> {
            if range.is_empty()
                || (self.sparse_table.len() > 0 && self.sparse_table[0].len() < range.end)
            {
                return None;
            }
            let loglen = self.logs2[range.end - range.start];
            let idx: usize = range.end - (1 << loglen);
            let a = self.sparse_table[loglen][range.start];
            let b = self.sparse_table[loglen][idx];
            Some((self.f)(a, b))
        }

        fn build_sparse_table(f: fn(T, T) -> T, input: &[T]) -> Vec<Vec<T>> {
            let len = input.len();
            let mut sparse_table: Vec<Vec<T>> = vec![vec![]; len.ilog2() as usize + 1];

            for i in 0..input.len() {
                sparse_table[0].push(input[i]);
            }

            for i in 1..=len.ilog2() as usize {
                let mut j = 0;
                while j + (1 << i) <= input.len() {
                    let a = sparse_table[i - 1][j];
                    let b = sparse_table[i - 1][j + (1 << (i - 1))];
                    sparse_table[i].push(f(a, b));
                    j += 1;
                }
            }
            sparse_table
        }
    }

    impl<T: Ord + Copy> RMQ<T> {
        pub fn max(input: &[T]) -> RMQ<T> {
            RMQ::new(max, input)
        }
    }

    impl<T: Ord + Copy> RMQ<T> {
        pub fn min(input: &[T]) -> RMQ<T> {
            RMQ::new(min, input)
        }
    }
}

struct Solution;

impl Solution {
    pub fn max_active_sections_after_trade(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let s = s.chars().collect::<Vec<_>>();
        let n = s.len();
        let all = s.iter().filter(|&c| *c == '1').count();

        let mut start = 0;
        let mut cnts = vec![];
        let mut cnt = 0;
        let mut prev = s[0];
        for (i, &c) in s.iter().enumerate() {
            if c == prev {
                cnt += 1;
            } else {
                cnts.push((start, i - 1, cnt, prev));
                start = i;
                cnt = 1;
                prev = c;
            }
        }
        cnts.push((start, s.len() - 1, cnt, prev));

        let mut moves = vec![0; n];
        for i in 0..cnts.len() {
            if cnts[i].3 == '1' && i > 0 && i < cnts.len() - 1 {
                moves[cnts[i].0] = cnts[i - 1].2 + cnts[i + 1].2;
            }
        }

        let rmq = cm_rmq::RMQ::max(&moves);

        queries
            .into_iter()
            .map(|q| {
                let (mut start, mut end) = (q[0] as usize, q[1] as usize);

                let mut start_ = cnts.partition_point(|cnt| cnt.0 <= start) - 1;
                let mut end_ = cnts.partition_point(|cnt| cnt.0 <= end) - 1;

                if start_ == end_ && cnts[start_].3 == '1' {
                    return 0;
                }

                let mut left_shifted = false;
                if cnts[start_].3 == '1' {
                    left_shifted = true;
                    start_ += 1;
                    start = cnts[start_].0;
                }
                let mut right_shifted = false;
                if cnts[end_].3 == '1' {
                    right_shifted = true;
                    end_ -= 1;
                    end = cnts[end_].1;
                }

                if end_ <= start_ {
                    return 0;
                }

                // 00|001100|00
                if end_ - start_ + 1 == 3 {
                    return (cnts[start_].1 - start + 1) + (end - cnts[end_].0 + 1);
                }

                let mut ans = 0;
                // 00|0011001100|011
                ans = ans.max((cnts[start_].1 - start + 1) + cnts[start_ + 2].2);
                ans = ans.max(cnts[end_ - 2].2 + (end - cnts[end_].0 + 1));

                if !left_shifted {
                    if start_ + 2 >= cnts.len() {
                        return ans;
                    } else {
                        start_ += 2;
                    }
                }
                if !right_shifted {
                    if end_ < 2 {
                        return ans;
                    } else {
                        end_ -= 2;
                    }
                }

                ans = ans.max(rmq.query(cnts[start_].0..cnts[end_].1).unwrap_or(0));

                return ans;
            })
            .map(|x| (all + x) as i32)
            .collect()
    }
}
