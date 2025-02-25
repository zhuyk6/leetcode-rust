pub struct Solution;

use crate::rctree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn range_sum_bst(root: Link, low: i32, high: i32) -> i32 {
        fn dfs(x: Link, l: i32, r: i32) -> i32 {
            x.map_or(0, |node| {
                let node = node.as_ref().borrow();
                dfs(node.left.clone(), l, r)
                    + dfs(node.right.clone(), l, r)
                    + if l <= node.val && node.val <= r {
                        node.val
                    } else {
                        0
                    }
            })
        }
        dfs(root.clone(), low, high)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::rctree::{TreeNode, tree};

    #[test]
    fn test1() {
        let root = tree![10, 5, 15, 3, 7, null, 18];
        let low = 7;
        let high = 15;

        assert_eq!(Solution::range_sum_bst(root, low, high), 32);
    }

    #[test]
    fn test2() {
        let root = tree![10, 5, 15, 3, 7, 13, 18, 1, null, 6];
        let low = 6;
        let high = 10;

        assert_eq!(Solution::range_sum_bst(root, low, high), 23);
    }
}
