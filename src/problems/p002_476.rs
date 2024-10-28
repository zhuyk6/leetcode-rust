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
    pub fn closest_nodes_treesearch(
        root: Option<Rc<RefCell<TreeNode>>>,
        queries: Vec<i32>,
    ) -> Vec<Vec<i32>> {
        fn lower_upper(x: Link, v: i32, l: &mut Option<i32>, r: &mut Option<i32>) {
            if let Some(node) = x {
                let node = node.borrow();
                match node.val.cmp(&v) {
                    std::cmp::Ordering::Less => {
                        *l = Some(node.val);
                        lower_upper(node.right.clone(), v, l, r);
                    }
                    std::cmp::Ordering::Equal => {
                        *l = Some(node.val);
                        *r = Some(node.val);
                    }
                    std::cmp::Ordering::Greater => {
                        *r = Some(node.val);
                        lower_upper(node.left.clone(), v, l, r);
                    }
                }
            }
        }

        queries
            .into_iter()
            .map(|v| {
                let mut l = None;
                let mut r = None;
                lower_upper(root.clone(), v, &mut l, &mut r);
                vec![l.unwrap_or(-1), r.unwrap_or(-1)]
            })
            .collect()
    }

    pub fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
        fn bst_dfs(x: Link, v: &mut Vec<i32>) {
            if let Some(node) = x {
                bst_dfs(node.borrow_mut().left.take(), v);
                v.push(node.borrow().val);
                bst_dfs(node.borrow_mut().right.take(), v);
            }
        }

        let mut v = vec![];
        bst_dfs(root, &mut v);
        let n = v.len();

        queries
            .into_iter()
            .map(|val| {
                let p = v.partition_point(|v| *v < val);
                let l = if p < n && v[p] == val {
                    val
                } else if p == 0 {
                    -1
                } else {
                    v[p - 1]
                };
                let r = if p == n { -1 } else { v[p] };
                vec![l, r]
            })
            .collect()
    }
}
