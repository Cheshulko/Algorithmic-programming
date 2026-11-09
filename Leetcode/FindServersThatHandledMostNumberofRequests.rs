// https://leetcode.com/problems/find-servers-that-handled-most-number-of-requests

use std::cmp::Ordering;

mod cm_seg_tree {
    use std::ops::Range;

    // [l, r)
    pub struct SegmentTree<T: Copy> {
        len: usize,
        tree: Vec<T>,
        merge: fn(T, T) -> T,
    }

    impl<T: Copy> SegmentTree<T>
    where
        T: std::cmp::PartialOrd,
    {
        // O(n)
        pub fn from_vec(arr: &[T], default: T, merge: fn(T, T) -> T) -> Self {
            let len = arr.len();

            let mut pow_2 = len.ilog2() as usize;
            if len & (len - 1) != 0 {
                pow_2 += 1;
            }
            let pow_2_len = 1 << pow_2;

            let mut sgtr = SegmentTree {
                len,
                tree: vec![default; 2 * pow_2_len - 1],
                merge,
            };

            sgtr.build_recursive(arr, 0, 0..len);
            sgtr
        }

        // range: [l, r). O(log(n))
        pub fn query(&self, range: Range<usize>, target: T) -> Option<T> {
            self.query_recursive(0, 0..self.len, &range, target)
        }

        // O(log(n))
        pub fn update(&mut self, idx: usize, val: T) {
            self.update_recursive(0, 0..self.len, idx, val);
        }

        fn build_recursive(&mut self, arr: &[T], idx: usize, range: Range<usize>) {
            if range.end - range.start == 1 {
                self.tree[idx] = arr[range.start];
            } else {
                let mid = range.start + (range.end - range.start) / 2;
                self.build_recursive(arr, 2 * idx + 1, range.start..mid);
                self.build_recursive(arr, 2 * idx + 2, mid..range.end);
                self.tree[idx] = (self.merge)(self.tree[2 * idx + 1], self.tree[2 * idx + 2]);
            }
        }

        fn query_recursive(
            &self,
            idx: usize,
            element_range: Range<usize>,
            query_range: &Range<usize>,
            target: T,
        ) -> Option<T> {
            if element_range.start >= query_range.end || element_range.end <= query_range.start {
                return None;
            }
            if !(self.tree[idx] <= target) {
                return None;
            }
            if element_range.end - element_range.start == 1 {
                return Some(self.tree[idx]);
            }
            let mid = element_range.start + (element_range.end - element_range.start) / 2;

            let left =
                self.query_recursive(idx * 2 + 1, element_range.start..mid, query_range, target);
            if let Some(left) = left {
                if left <= target {
                    return Some(left);
                }
            }

            let right =
                self.query_recursive(idx * 2 + 2, mid..element_range.end, query_range, target);
            if let Some(right) = right {
                if right <= target {
                    return Some(right);
                }
            }

            return None;
        }

        fn update_recursive(
            &mut self,
            idx: usize,
            element_range: Range<usize>,
            element_idx: usize,
            val: T,
        ) {
            if element_range.start > element_idx || element_range.end <= element_idx {
                return;
            }
            if element_range.end - element_range.start == 1 && element_range.start == element_idx {
                self.tree[idx] = val;
                return;
            }
            let mid = element_range.start + (element_range.end - element_range.start) / 2;
            self.update_recursive(idx * 2 + 1, element_range.start..mid, element_idx, val);
            self.update_recursive(idx * 2 + 2, mid..element_range.end, element_idx, val);
            self.tree[idx] = (self.merge)(self.tree[idx * 2 + 1], self.tree[idx * 2 + 2]);
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Node {
    time: i64,
    ind: usize,
}

impl Node {
    pub fn new(time: i64, ind: usize) -> Self {
        Self { time, ind }
    }

    pub fn new_ind(ind: usize) -> Self {
        Self { time: 0, ind }
    }

    pub fn max() -> Self {
        Self {
            time: i64::MAX,
            ind: usize::MAX,
        }
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.time.partial_cmp(&other.time) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.ind.partial_cmp(&other.ind)
    }
}

struct Solution;

impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        let k = k as usize;

        let arr = (0..k).map(Node::new_ind).collect::<Vec<_>>();

        let mut tree: cm_seg_tree::SegmentTree<Node> =
            cm_seg_tree::SegmentTree::from_vec(&arr, Node::max(), |a, b| {
                if a.time <= b.time {
                    a
                } else {
                    b
                }
            });

        let mut count = vec![0; k];

        for (i, (arrival, load)) in arrival.into_iter().zip(load.into_iter()).enumerate() {
            let i = i % k;
            let target = Node::new(arrival as i64, usize::MAX);

            if let Some(mut next) = tree.query(i..k, target.clone()) {
                count[next.ind] += 1;
                next.time = (arrival as i64 + load as i64);
                tree.update(next.ind, next);

                continue;
            }

            if let Some(mut next) = tree.query(0..i, target.clone()) {
                count[next.ind] += 1;
                next.time = (arrival as i64 + load as i64);
                tree.update(next.ind, next);

                continue;
            }
        }

        let ma = count.iter().max().copied().unwrap();

        count
            .into_iter()
            .enumerate()
            .filter_map(|(i, x)| (x == ma).then_some(i as i32))
            .collect()
    }
}
