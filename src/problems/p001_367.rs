use std::cell::RefCell;
use std::rc::Rc;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs2(node: Option<Rc<RefCell<TreeNode>>>, head: Option<&ListNode>) -> bool {
            if let Some(head) = head {
                if let Some(node) = node {
                    let node = node.borrow();
                    node.val == head.val
                        && (dfs2(node.left.clone(), head.next.as_deref())
                            || dfs2(node.right.clone(), head.next.as_deref()))
                } else {
                    false
                }
            } else {
                true
            }
        }

        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, head: Option<&ListNode>) -> bool {
            if dfs2(node.clone(), head) {
                return true;
            }
            if let Some(node) = node {
                let node = node.borrow();
                dfs(node.left.clone(), head) || dfs(node.right.clone(), head)
            } else {
                false
            }
        }

        dfs(root, head.as_deref())
    }
}
