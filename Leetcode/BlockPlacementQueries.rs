// https://leetcode.com/problems/block-placement-queries

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

struct Solution;

impl Solution {
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        use std::collections::BTreeSet;

        let ma = queries.iter().map(|q| q[1] as usize).max().unwrap();

        let mut obstacles = BTreeSet::<usize>::new();
        obstacles.insert(0);
        obstacles.insert(ma + 1);

        let mut sizes = vec![usize::MIN; ma + 2];
        sizes[0] = 0;
        sizes[ma + 1] = ma + 1;

        let mut tree = cm_seg_tree::SegmentTree::from_vec(&sizes, usize::MIN, |a, b| a.max(b));
        tree.update(0, 0);
        tree.update(ma + 1, ma + 1);

        let mut ans = vec![];
        for query in queries {
            match query[0] {
                1 => {
                    let x = query[1] as usize;
                    let right = obstacles.range(x..).next().copied().unwrap();
                    let size = sizes[right];
                    let start = right - size;
                    let left_size = x - start;
                    let right_size = size - left_size;

                    tree.update(x, left_size);
                    tree.update(right, right_size);

                    sizes[x] = left_size;
                    sizes[right] = right_size;

                    obstacles.insert(x);
                }
                2 => {
                    let (x, size) = (query[1] as usize, query[2] as usize);
                    let left = obstacles.range(..(x + 1)).next_back().copied().unwrap();
                    let ma = tree.query(0..(left + 1)).unwrap_or_default();

                    ans.push(ma >= size || x - left >= size);
                }
                _ => unreachable!(),
            }
        }

        ans
    }
}
