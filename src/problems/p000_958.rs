use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

use crate::rctree::TreeNode;
use crate::tree;
struct Solution;

impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        type Link = Option<Rc<RefCell<TreeNode>>>;

        let mut que: VecDeque<(Link, usize)> = Default::default();
        let mut layers: Vec<Vec<bool>> = vec![];

        que.push_back((root.clone(), 0));
        while let Some((t, dep)) = que.pop_front() {
            if dep >= layers.len() {
                layers.push(vec![]);
            }
            layers[dep].push(t.is_some());

            if let Some(node) = t {
                que.push_back((node.as_ref().borrow().left.clone(), dep+1));
                que.push_back((node.as_ref().borrow().right.clone(), dep+1));
            }
        }

        println!("{:#?}", layers);

        let max_dep = layers.len();

        let b1 = layers[..(max_dep-2)].iter().all(|v| v.iter().all(|b| *b));
        let b2 = {
            let v = &layers[max_dep-2];
            let mut last = v.len()-1;
            while !v[last] {last -= 1;}
            v[..=last].iter().all(|b| *b)
        };
        println!("b1 = {}, b2 = {}", b1,b2);
        b1 && b2
    }
}

#[test]
fn example() {
    let t = tree![1, 2, 3, 4, 5, 6];

    assert_eq!(Solution::is_complete_tree(t), true);

    let t = tree![1,2,3,4,5,null,7];
    assert_eq!(Solution::is_complete_tree(t), false);
}
