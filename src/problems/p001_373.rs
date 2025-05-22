pub struct Solution;
use crate::rctree::TreeNode;

fn dfs(node: &RefCell<TreeNode>, ans: &mut i32) -> Option<(i32, i32, i32)> {
    let mut ret = Some((node.borrow().val, node.borrow().val, node.borrow().val));

    if let Some(left) = node.borrow().left.as_deref() {
        if let Some((l_min, l_max, l_sum)) = dfs(left, ans) {
            if l_max >= node.borrow().val {
                ret = None;
            }
            if let Some((min, _, sum)) = ret.as_mut() {
                *min = l_min;
                *sum += l_sum;
            }
        } else {
            ret = None;
        }
    }

    if let Some(right) = node.borrow().right.as_deref() {
        if let Some((r_min, r_max, r_sum)) = dfs(right, ans) {
            if r_min <= node.borrow().val {
                ret = None;
            }
            if let Some((_, max, sum)) = ret.as_mut() {
                *max = r_max;
                *sum += r_sum;
            }
        } else {
            ret = None;
        }
    }

    // update answer
    if let Some((_, _, sum)) = ret.as_ref() {
        *ans = (*ans).max(*sum);
    }

    ret
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        if let Some(node) = root {
            dfs(&node, &mut ans);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rctree::tree;

    #[test]
    fn sample1() {
        let root = tree![
            1, 4, 3, 2, 4, 2, 5, null, null, null, null, null, null, 4, 6
        ];
        assert_eq!(Solution::max_sum_bst(root), 20);
    }

    #[test]
    fn sample2() {
        let root = tree![4, 3, null, 1, 2];
        assert_eq!(Solution::max_sum_bst(root), 2);
    }

    #[test]
    fn sample3() {
        let root = tree![-4, -2, -5];
        assert_eq!(Solution::max_sum_bst(root), 0);
    }

    #[test]
    fn sample4() {
        let root = tree![2, 1, 3];
        assert_eq!(Solution::max_sum_bst(root), 6);
    }

    #[test]
    fn sample5() {
        let root = tree![5, 4, 8, 3, null, 6, 3];
        assert_eq!(Solution::max_sum_bst(root), 7);
    }
}
