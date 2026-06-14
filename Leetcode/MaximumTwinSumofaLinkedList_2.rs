// https://leetcode.com/problems/maximum-twin-sum-of-a-linked-list

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

struct Solution;

impl Solution {
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        use std::collections::VecDeque;

        fn go(head: Option<&Box<ListNode>>, st: &mut VecDeque<i32>) -> i32 {
            let ans = if let Some(head) = head {
                st.push_back(head.val);

                let ans = go(head.next.as_ref(), st);
                ans.max(st.pop_front().unwrap() + head.val)
            } else {
                0
            };

            ans
        }

        let mut st = VecDeque::new();
        go(head.as_ref(), &mut st)
    }
}
