use crate::rctree::{TreeNode};
struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn del_nodes(root: Option<Rc<RefCell<TreeNode>>>, to_delete: Vec<i32>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        type Link = Option<Rc<RefCell<TreeNode>>>;
        use std::collections::{HashSet, VecDeque};

        let to_delete: HashSet<i32> = HashSet::from_iter(to_delete);
        let mut que: VecDeque<( Link, Link, bool )> = VecDeque::new();
        que.push_back(( root.clone(), None , false));

        let mut ans = vec![];

        while let Some((t, fa, d)) = que.pop_front() {
            if let Some(node) = t {
                let new_fa = if to_delete.contains(&node.as_ref().borrow().val) {
                    if let Some(fa_node) = fa {
                        if !d {
                            fa_node.as_ref().borrow_mut().left = None;
                        } else {
                            fa_node.as_ref().borrow_mut().right = None;
                        }    
                    }
                    None
                } else {
                    if fa.is_none() {
                        ans.push(Some(node.clone()));
                    }
                    Some(node.clone())
                };
                que.push_back((node.as_ref().borrow().left.clone(), new_fa.clone(), false));
                que.push_back((node.as_ref().borrow().right.clone(), new_fa.clone(), true));
            }
        }
        ans
    }
}

#[test]
fn example() {
    let root = tree![1,2,3,4,5,6,7]; 
    let to_delete = vec![3,5, 1];
    let ans = Solution::del_nodes(root, to_delete);
    for t in ans {
        println!("t: {:?}", TreeNode::to_list(t));
    }
}
