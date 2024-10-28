use crate::rctree::TreeNode;
pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

struct Dfs {
    start: i32,
    ans: u32,
}

impl Dfs {
    fn dfs(&mut self, x: Option<Rc<RefCell<TreeNode>>>) -> (u32, Option<u32>) {
        if let Some(node) = x {
            if node.as_ref().borrow().val == self.start {
                let (d1, _) = self.dfs(node.as_ref().borrow().left.clone());
                let (d2, _) = self.dfs(node.as_ref().borrow().right.clone());
                self.ans = self.ans.max(d1.max(d2));

                (d1.max(d2) + 1, Some(1))
            } else {
                let (d1, option1) = self.dfs(node.as_ref().borrow().left.clone());
                let (d2, option2) = self.dfs(node.as_ref().borrow().right.clone());

                println!("{d1}, {option1:?}");
                println!("{d2}, {option2:?}");

                if let Some(d0) = option1 {
                    self.ans = self.ans.max(d0 + 1 + d2);

                    (d1.max(d2) + 1, Some(d0 + 1))
                } else if let Some(d0) = option2 {
                    self.ans = self.ans.max(d0 + 1 + d1);

                    (d1.max(d2) + 1, Some(d0 + 1))
                } else {
                    (d1.max(d2) + 1, None)
                }
            }
        } else {
            (0, None)
        }
    }
}

impl Solution {
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        let mut dfs = Dfs { start, ans: 0 };
        dfs.dfs(root);
        dfs.ans as i32
    }
}
