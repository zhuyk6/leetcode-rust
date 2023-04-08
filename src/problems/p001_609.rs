
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
       right: None
     }
   }
 }
struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
		let mut que = VecDeque::new();
		let mut layers = Vec::new();

		que.push_back((root, 0usize));
		while let Some((node, d)) = que.pop_front() {
			if let Some(node) = node {
				if layers.len() <= d {
					layers.push(Vec::new());
				}
				layers[d].push(node.borrow().val);
				que.push_back((node.borrow().left.clone(), d + 1));
				que.push_back((node.borrow().right.clone(), d + 1));
			}
		}

		fn is_sorted<T: PartialOrd>(v: &Vec<T>, rev: bool) -> bool {
			match rev {
                false => v.iter().zip(v.iter().skip(1)).all(|(x, y)| x < y),
				true  => v.iter().zip(v.iter().skip(1)).all(|(x, y)| y < x),
            }
		}
		layers.into_iter().enumerate().all(|(d, v)| 
			is_sorted(&v, d & 1 == 1) 
			&& v.into_iter().all(|x| x & 1 != (d & 1) as i32 )
		)
    }
}