// https://leetcode.com/problems/create-binary-tree-from-descriptions

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::HashMap;

        let mut nodes_map = HashMap::<i32, Rc<RefCell<TreeNode>>>::new();

        let mut parents = HashMap::<i32, i32>::new();

        for description in descriptions {
            let &[parent, child, is_left] = description.as_slice() else {
                unreachable!()
            };

            parents.insert(child, parent);

            let child = nodes_map
                .entry(child)
                .or_insert(Rc::new(RefCell::new(TreeNode {
                    val: child,
                    left: None,
                    right: None,
                })));

            let child = child.clone();

            let parent = nodes_map
                .entry(parent)
                .or_insert(Rc::new(RefCell::new(TreeNode {
                    val: parent,
                    left: None,
                    right: None,
                })));

            let mut parent = parent.borrow_mut();
            if is_left == 1 {
                parent.left = Some(child);
            } else {
                parent.right = Some(child);
            }
        }

        let Some(mut root) = nodes_map.iter().next().map(|(v, _)| v) else {
            return None;
        };

        while let Some(parent) = parents.get(root) {
            root = parent;
        }

        nodes_map.get(root).cloned()
    }
}
