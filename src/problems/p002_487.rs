struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
#[allow(unused)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

type Link = Option<Box<ListNode>>;

#[allow(unused)]
impl Solution {
    pub fn remove_nodes(head: Link) -> Link {
        fn dfs(cur: Link) -> Link {
            cur.map(|mut node| {
                if let Some(next) = dfs(node.next.take()) {
                    if node.val < next.val {
                        next
                    } else {
                        node.next = Some(next);
                        node
                    }
                } else {
                    node
                }
            })
        }

        dfs(head)
    }
}
