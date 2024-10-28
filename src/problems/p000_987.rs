use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;

use crate::rctree::TreeNode;
pub struct Solution;

type Link = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut que: VecDeque<(Link, usize, i32)> = VecDeque::new();
        let mut layers: Vec<Vec<(i32, i32)>> = Vec::new();

        que.push_back((root, 0, 0));
        while let Some((t, r, c)) = que.pop_front() {
            if let Some(node) = t {
                if r >= layers.len() {
                    layers.push(Vec::new());
                }
                layers[r].push((c, node.borrow().val));
                que.push_back((node.borrow_mut().left.take(), r + 1, c - 1));
                que.push_back((node.borrow_mut().right.take(), r + 1, c + 1));
            }
        }
        for row in &mut layers {
            row.sort();
        }

        println!("{:#?}", layers);

        let mut cols: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
        for row in layers {
            for (col, v) in row {
                let e = cols.entry(col).or_default();
                e.push(v);
            }
        }
        cols.into_values().collect()
    }
}
