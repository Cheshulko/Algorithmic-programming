// https://leetcode.com/problems/delete-the-middle-node-of-a-linked-list

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
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut cur = head.as_ref();
        while let Some(cur_) = cur {
            len += 1;
            cur = cur_.next.as_ref();
        }

        fn go(head: Option<Box<ListNode>>, n: usize) -> Option<Box<ListNode>> {
            let Some(mut head) = head else {
                return None;
            };

            if n > 0 {
                let next = go(head.as_mut().next.take(), n - 1);
                head.next = next;
                Some(head)
            } else {
                head.next
            }
        }

        go(head, len >> 1)
    }
}
