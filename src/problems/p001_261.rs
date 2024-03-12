use crate::rctree::TreeNode;
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

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

type Link = Rc<RefCell<TreeNode>>;

struct FindElements {
    map: HashSet<i32>,
}

#[allow(unused)]
impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        fn dfs(x: Option<Link>, v: i32, map: &mut HashSet<i32>) {
            if let Some(node) = x {
                map.insert(v);
                node.as_ref().borrow_mut().val = v;
                dfs(node.as_ref().borrow().left.clone(), v * 2 + 1, map);
                dfs(node.as_ref().borrow().right.clone(), v * 2 + 2, map);
            }
        }

        let mut map = HashSet::new();
        dfs(root.clone(), 0, &mut map);
        FindElements { map }
    }

    fn find(&self, target: i32) -> bool {
        self.map.get(&target).is_some()
    }
}
