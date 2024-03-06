use crate::rctree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Solution;

type Link = Option<Rc<RefCell<TreeNode>>>;

fn get_height(t: Link, dep: usize) -> usize {
    t.map_or(0, |node| {
        let lh = get_height(node.borrow().left.clone(), dep + 1);
        let rh = get_height(node.borrow().right.clone(), dep + 1);
        dep.max(lh.max(rh))
    })
}

#[inline]
fn pow2(n: usize) -> usize {
    1 << n
}

#[allow(unused)]
impl Solution {
    pub fn print_tree(root: Link) -> Vec<Vec<String>> {
        let h = get_height(root.clone(), 0);
        let m = h + 1;
        let n = pow2(m) - 1;
        let mut res = vec![vec!["".to_string(); n]; m];

        let mut que: VecDeque<(Link, usize, usize)> = VecDeque::new();
        que.push_back((root.clone(), 0, (n - 1) / 2));

        while let Some((t, r, c)) = que.pop_front() {
            if let Some(node) = t {
                res[r][c] = node.borrow().val.to_string();
                que.push_back((node.borrow().left.clone(), r + 1, c - pow2(h - r - 1)));
                que.push_back((node.borrow().right.clone(), r + 1, c + pow2(h - r - 1)));
            }
        }

        res
    }
}
