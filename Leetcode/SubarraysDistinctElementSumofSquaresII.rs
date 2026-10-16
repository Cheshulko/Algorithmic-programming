// https://leetcode.com/problems/subarrays-distinct-element-sum-of-squares-ii

mod cm_lazy_seg_tree_capture {
    use std::ops::Range;

    // T - node value, TL - lazy value
    // [l, r)
    pub struct LazySegmentTree<F, M, L, T: Copy, TL: Copy>
    where
        F: Fn(&T, &T) -> T,                 // left, right
        M: Fn(&TL, &TL) -> TL,              // old lazy, new lazy
        L: Fn(&T, &Range<usize>, &TL) -> T, // node value, node range, lazy value
    {
        len: usize,
        tree: Vec<T>,
        lazy: Vec<Option<TL>>,
        merge: F,
        merge_lazy: M,
        apply_lazy: L,
    }

    impl<F, M, L, T: Copy, TL: Copy> LazySegmentTree<F, M, L, T, TL>
    where
        F: Fn(&T, &T) -> T,
        M: Fn(&TL, &TL) -> TL,
        L: Fn(&T, &Range<usize>, &TL) -> T,
    {
        // O(n)
        pub fn from_vec(arr: &[T], default: T, merge: F, merge_lazy: M, apply_lazy: L) -> Self {
            let len = arr.len();

            let mut pow_2 = len.ilog2() as usize;
            if len & (len - 1) != 0 {
                pow_2 += 1;
            }
            let pow_2_len = 1 << pow_2;

            let mut sgtr = LazySegmentTree {
                len,
                tree: vec![default; 2 * pow_2_len - 1],
                lazy: vec![None; 2 * pow_2_len - 1],
                merge,
                merge_lazy,
                apply_lazy,
            };

            sgtr.build_recursive(arr, 0, 0..len);
            sgtr
        }

        // range: [l, r). O(log(n))
        pub fn query(&mut self, range: Range<usize>) -> Option<T> {
            self.query_recursive(0, 0..self.len, &range)
        }

        // range: [l, r). O(log(n))
        pub fn update(&mut self, range: Range<usize>, lazy_val: TL) {
            self.update_recursive(0, 0..self.len, &range, lazy_val);
        }

        fn build_recursive(&mut self, arr: &[T], idx: usize, range: Range<usize>) {
            if range.end - range.start == 1 {
                self.tree[idx] = arr[range.start];
            } else {
                let mid = range.start + (range.end - range.start) / 2;
                self.build_recursive(arr, 2 * idx + 1, range.start..mid);
                self.build_recursive(arr, 2 * idx + 2, mid..range.end);
                self.tree[idx] = (self.merge)(&self.tree[2 * idx + 1], &self.tree[2 * idx + 2]);
            }
        }

        fn query_recursive(
            &mut self,
            idx: usize,
            element_range: Range<usize>,
            query_range: &Range<usize>,
        ) -> Option<T> {
            if element_range.start >= query_range.end || element_range.end <= query_range.start {
                return None;
            }
            if element_range.start >= query_range.start && element_range.end <= query_range.end {
                return Some(self.tree[idx]);
            }

            let mid = element_range.start + (element_range.end - element_range.start) / 2;
            self.propagate(idx, &element_range);
            let left = self.query_recursive(idx * 2 + 1, element_range.start..mid, query_range);
            let right = self.query_recursive(idx * 2 + 2, mid..element_range.end, query_range);
            match (left, right) {
                (None, None) => None,
                (None, Some(r)) => Some(r),
                (Some(l), None) => Some(l),
                (Some(l), Some(r)) => Some((self.merge)(&l, &r)),
            }
        }

        fn update_recursive(
            &mut self,
            idx: usize,
            element_range: Range<usize>,
            update_range: &Range<usize>,
            lazy_val: TL,
        ) {
            if element_range.start >= update_range.end || element_range.end <= update_range.start {
                return;
            }
            if element_range.end - element_range.start == 1 {
                self.apply(idx, &element_range, lazy_val);
                return;
            }
            if element_range.start >= update_range.start && element_range.end <= update_range.end {
                self.apply(idx, &element_range, lazy_val);
                return;
            }

            let mid = element_range.start + (element_range.end - element_range.start) / 2;
            self.propagate(idx, &element_range);
            self.update_recursive(
                idx * 2 + 1,
                element_range.start..mid,
                update_range,
                lazy_val,
            );
            self.update_recursive(idx * 2 + 2, mid..element_range.end, update_range, lazy_val);
            self.tree[idx] = (self.merge)(&self.tree[idx * 2 + 1], &self.tree[idx * 2 + 2]);
        }

        fn apply(&mut self, idx: usize, element_range: &Range<usize>, lazy_val: TL) {
            self.tree[idx] = (self.apply_lazy)(&self.tree[idx], element_range, &lazy_val);
            self.lazy[idx] = match self.lazy[idx] {
                Some(lazy) => Some((self.merge_lazy)(&lazy, &lazy_val)),
                None => Some(lazy_val),
            };
        }

        fn propagate(&mut self, idx: usize, element_range: &Range<usize>) {
            if let Some(lazy) = self.lazy[idx] {
                let mid = element_range.start + (element_range.end - element_range.start) / 2;
                self.apply(2 * idx + 1, &(element_range.start..mid), lazy);
                self.apply(2 * idx + 2, &(mid..element_range.end), lazy);
            }
            self.lazy[idx] = None;
        }
    }
}

#[derive(Default, Clone, Copy)]
struct Node {
    sum: i64,
    sum_sqr: i64,
}

struct Solution;

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        const M: i64 = 1_000_000_000 + 7;

        let mut tree = cm_lazy_seg_tree_capture::LazySegmentTree::from_vec(
            &vec![Node::default(); n + 1],
            Node::default(),
            |&left, &right| Node {
                sum: (left.sum + right.sum) % M,
                sum_sqr: (left.sum_sqr + right.sum_sqr) % M,
            },
            |&lazy_1: &i64, &lazy_2| (lazy_1 + lazy_2) % M,
            |&node_val, element_range, &lazy| {
                /*
                node_val = x1^2 + x2^2 + .. + xn^2
                (x1 + lazy)^2 + (x2 + lazy)^2 + .. + (xn + lazy)^2 =
                   =  x1^2 + x2^2 + .. + xn^2 + 2*x1*lazy + 2*x2*lazy+ .. + 2*xn*lazy + n*lazy^2
                   = node_val + n*lazy^2 + 2*lazy(x1 + x2 + .. + xn)
                */

                let n = element_range.len() as i64;
                Node {
                    sum: (node_val.sum + n * lazy) % M,
                    sum_sqr: (node_val.sum_sqr + n * lazy * lazy + 2 * lazy * node_val.sum) % M,
                }
            },
        );

        use std::collections::HashMap;

        let mut seen = HashMap::new();
        let mut ans = 0;
        for (i, x) in nums.into_iter().enumerate() {
            let x = x as i64;
            let i = i + 1;

            let j = seen.get(&x).copied().unwrap_or_default();
            tree.update(j + 1..i + 1, 1);

            let v = tree.query(1..i + 1).unwrap().sum_sqr;
            ans = (ans + v) % M;

            seen.insert(x, i);
        }

        ans as i32
    }
}
