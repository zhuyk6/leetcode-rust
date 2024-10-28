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

pub struct Solution;

type Link = Option<Box<ListNode>>;

impl Solution {
    pub fn trainning_plan(mut l1: Link, mut l2: Link) -> Link {
        // fn dfs(mut l1: Link, mut l2: Link) -> Link {
        //     if let Some(n1) = l1.as_mut() {
        //         if let Some(n2) = l2.as_mut() {
        //             let (n3, v) = if n1.val <= n2.val {
        //                 (dfs(n1.next.take(), l2), n1.val)
        //             } else {
        //                 (dfs(l1, n2.next.take()), n2.val)
        //             };
        //             let mut node = ListNode::new(v);
        //             node.next = n3;
        //             Some(Box::new(node))
        //         } else {
        //             l1
        //         }
        //     } else {
        //         l2
        //     }
        // }
        // dfs(l1, l2)

        let mut head = ListNode::new(0);
        let mut cur = &mut head;

        while l1.is_some() && l2.is_some() {
            if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
                let mut node = l1.unwrap();
                l1 = node.next.take();
                cur.next = Some(node);
            } else {
                let mut node = l2.unwrap();
                l2 = node.next.take();
                cur.next = Some(node);
            }
            cur = cur.next.as_deref_mut().unwrap();
        }

        if l1.is_some() {
            cur.next = l1;
        }
        if l2.is_some() {
            cur.next = l2;
        }

        head.next.take()
    }
}
