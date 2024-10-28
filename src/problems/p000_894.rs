use crate::rctree::TreeNode;
pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn build(n: usize) -> Vec<Rc<RefCell<TreeNode>>> {
            match n {
                0 => vec![],
                1 => vec![Rc::new(RefCell::new(TreeNode::new(0)))],
                _ => {
                    let mut ans = vec![];

                    for i in 1..(n - 1) {
                        let arr_left = build(i);
                        let arr_right = build(n - i - 1);

                        println!(
                            "n = {n}, i = {i}, left = {}, right = {}",
                            arr_left.len(),
                            arr_right.len()
                        );

                        for left in &arr_left {
                            for right in &arr_right {
                                let root = Rc::new(RefCell::new(TreeNode::new(0)));
                                root.as_ref().borrow_mut().left = Some(left.clone());
                                root.as_ref().borrow_mut().right = Some(right.clone());
                                ans.push(root);
                            }
                        }
                    }

                    ans
                }
            }
        }

        build(n as usize).into_iter().map(Some).collect()
    }
}
