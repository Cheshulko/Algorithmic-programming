// https://leetcode.com/problems/jump-game-ix

use std::collections::HashMap;

mod cm_seg_tree {
    use std::ops::Range;

    // [l, r)
    pub struct SegmentTree<T: Copy> {
        len: usize,
        tree: Vec<T>,
        merge: fn(T, T) -> T,
    }

    impl<T: Copy> SegmentTree<T> {
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
        pub fn query(&self, range: Range<usize>) -> Option<T> {
            self.query_recursive(0, 0..self.len, &range)
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
        ) -> Option<T> {
            if element_range.start >= query_range.end || element_range.end <= query_range.start {
                return None;
            }
            if element_range.start >= query_range.start && element_range.end <= query_range.end {
                return Some(self.tree[idx]);
            }
            let mid = element_range.start + (element_range.end - element_range.start) / 2;
            let left = self.query_recursive(idx * 2 + 1, element_range.start..mid, query_range);
            let right = self.query_recursive(idx * 2 + 2, mid..element_range.end, query_range);
            match (left, right) {
                (None, None) => None,
                (None, Some(r)) => Some(r),
                (Some(l), None) => Some(l),
                (Some(l), Some(r)) => Some((self.merge)(l, r)),
            }
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

fn compress<
    T: From<i8>
        + Copy
        + std::hash::Hash
        + std::cmp::Eq
        + std::cmp::Ord
        + std::ops::Sub<T, Output = T>
        + std::ops::Add<T, Output = T>,
>(
    input: Vec<T>,
) -> HashMap<T, usize> {
    use std::collections::{BTreeSet, HashMap};

    let mut hs = BTreeSet::<T>::new();
    let mut mp = HashMap::<T, usize>::new();

    for &x in input.iter() {
        hs.insert(x);
        hs.insert(x - 1_i8.into());
        hs.insert(x + 1_i8.into());
    }
    for x in hs.into_iter() {
        if !mp.contains_key(&x) {
            let len = mp.len();
            mp.insert(x, len);
        }
    }

    mp
}

struct Solution;

impl Solution {
    pub fn max_value(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::BTreeSet;

        let n = nums.len();

        let mut pref = nums
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<BTreeSet<_>>();

        let compressed = compress(nums.clone());
        let n_compressed = compressed.len();

        let mut tree =
            cm_seg_tree::SegmentTree::from_vec(&vec![i32::MIN; n_compressed], i32::MIN, |a, b| {
                a.max(b)
            });

        let mut ans = vec![i32::MIN; n];
        for (i, num) in nums.into_iter().enumerate().rev() {
            ans[i] = num;

            if let Some(&(max_pref, _)) = pref.last() {
                ans[i] = ans[i].max(max_pref);

                let max_pref_compressed = compressed.get(&max_pref).copied().unwrap();

                if let Some(tail_max) = tree.query(0..max_pref_compressed) {
                    ans[i] = ans[i].max(tail_max);
                }
            }

            let num_compressed = compressed.get(&num).copied().unwrap();
            tree.update(num_compressed, ans[i]);

            pref.remove(&(num, i));
        }

        ans
    }
}
