// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use crate::rctree::TreeNode;
pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        fn get_dep_fa(x: Link, val: i32, dep: u32, fa: Option<i32>) -> Option<(u32, Option<i32>)> {
            x.and_then(|node| {
                let node = node.borrow();
                if node.val == val {
                    Some((dep, fa))
                } else {
                    get_dep_fa(node.left.clone(), val, dep + 1, Some(node.val)).or(get_dep_fa(
                        node.right.clone(),
                        val,
                        dep + 1,
                        Some(node.val),
                    ))
                }
            })
        }

        let (dep_x, fa_x) = get_dep_fa(root.clone(), x, 0, None).unwrap();
        let (dep_y, fa_y) = get_dep_fa(root.clone(), y, 0, None).unwrap();

        dep_x == dep_y && fa_x != fa_y
    }
}
