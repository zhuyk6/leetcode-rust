use std::rc::Rc;
use std::cell::RefCell;

use crate::rctree::TreeNode;

type Link = Option<Rc<RefCell<TreeNode>>>;

struct Solution;

impl Solution {
    pub fn sufficient_subset(root: Link, limit: i32) -> Link {
        fn dfs(t: Link, acc: i32, limit: i32) -> Link {
            if let Some(node) = t {
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    if acc + node.borrow().val < limit {
                        None
                    } else {
                        Some(node)
                    }
                } else {
                    let acc = acc + node.borrow().val;
                    let lc = dfs(node.borrow_mut().left.take(), acc, limit);
                    let rc = dfs(node.borrow_mut().right.take(), acc, limit);
                    if lc.is_none() && rc.is_none() {
                        None
                    } else {
                        node.borrow_mut().left = lc;
                        node.borrow_mut().right = rc;
                        Some(node)
                    }
                }
            } else {
                None
            }
        }

        dfs(root, 0, limit)
    }
}

