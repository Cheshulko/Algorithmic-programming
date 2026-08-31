// https://leetcode.com/problems/find-the-minimum-and-maximum-number-of-nodes-between-critical-points

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        fn go(head: Option<Box<ListNode>>, prev: Option<i32>, p: usize, ps: &mut Vec<usize>) {
            let Some(head) = head else {
                return;
            };

            let cur = head.val;
            if let (Some(next), Some(prev)) = (head.next.as_ref().map(|n| n.val), prev) {
                if (prev < cur && cur > next) || (prev > cur && cur < next) {
                    ps.push(p);
                }
            }

            go(head.next, Some(cur), p + 1, ps);
        }

        let mut ps = vec![];
        go(head, None, 0, &mut ps);
        if ps.len() <= 1 {
            return vec![-1, -1];
        }

        let mi = ps.windows(2).map(|w| w[1] - w[0]).min().unwrap();
        let ma = ps[ps.len() - 1] - ps[0];

        vec![mi as i32, ma as i32]
    }
}
