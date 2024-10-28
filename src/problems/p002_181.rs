use crate::boxlist::ListNode;

pub struct Solution;

type Link = Option<Box<ListNode>>;

impl Solution {
    #[allow(dead_code)]
    fn merge_nodes_loop(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut acc = 0;
        let mut cur = head;
        let mut head = None;
        let mut tail: Option<&mut ListNode> = None;

        while let Some(node) = cur.take() {
            if node.val == 0 {
                if acc > 0 {
                    // insert a node, which value is `acc`
                    let node = Box::new(ListNode::new(acc));
                    if let Some(prev) = tail {
                        prev.next = Some(node);
                        tail = prev.next.as_deref_mut();
                    } else {
                        head = Some(node);
                        tail = head.as_deref_mut();
                    }
                }
                acc = 0;
            } else {
                acc += node.val;
            }
            cur = node.next;
        }
        head
    }

    fn merge_nodes_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn f(link: Link, acc: i32) -> Link {
            link.map(|node| {
                if node.val == 0 {
                    let mut new_node = Box::new(ListNode::new(acc));
                    new_node.next = f(node.next, 0);
                    new_node
                } else {
                    f(node.next, acc + node.val).unwrap()
                }
            })
        }
        f(head.unwrap().next, 0)
    }
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::merge_nodes_recursive(head)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boxlist::list;

    #[test]
    fn sample1() {
        let head = list![0, 3, 1, 0, 4, 5, 2, 0];
        let ans = list![4, 11];
        assert_eq!(Solution::merge_nodes(head), ans);
    }

    #[test]
    fn sample2() {
        let head = list![0, 1, 0, 3, 0, 2, 2, 0];
        let ans = list![1, 3, 4];
        assert_eq!(Solution::merge_nodes(head), ans);
    }
}
