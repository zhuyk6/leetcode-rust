use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

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

type Link = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn width_of_binary_tree(root: Link) -> i32 {
        let mut que: VecDeque<(Link, usize, u32)> = VecDeque::new();
        let mut cur_layer = 0;
        let mut layers_min: u32 = 0;
        let mut ans = 0;
        que.push_back((root, 1, 1));
        while let Some((t, d, idx)) = que.pop_front() {
            if let Some(node) = t {
                if d > cur_layer {
                    layers_min = idx;
                    cur_layer = d;
                }
                ans = ans.max(idx - layers_min);
                que.push_back((node.as_ref().borrow().left.clone(), d + 1, idx * 2));
                que.push_back((node.as_ref().borrow().right.clone(), d + 1, idx * 2 + 1));
            }
        }
        (ans + 1) as i32
    }
}
