use crate::rctree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

type Link = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn prune_tree(root: Link) -> Link {
        if let Some(node) = root {
            let lc = Solution::prune_tree(node.as_ref().borrow_mut().left.take());
            let rc = Solution::prune_tree(node.as_ref().borrow_mut().right.take());
            let mut has_subtree = false;
            if lc.is_some() {
                node.as_ref().borrow_mut().left = lc;
                has_subtree = true;
            }
            if rc.is_some() {
                node.as_ref().borrow_mut().right = rc;
                has_subtree = true;
            }
            if !has_subtree && node.as_ref().borrow().val == 0 {
                None
            } else {
                Some(node)
            }
        } else {
            None
        }
    }
}
