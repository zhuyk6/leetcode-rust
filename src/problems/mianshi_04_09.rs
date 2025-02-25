use crate::rctree::TreeNode;
pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn merge(a: &[i32], b: &[i32], seq: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if a.is_empty() && b.is_empty() {
            ans.push(seq.clone());
        } else {
            if !a.is_empty() {
                seq.push(a[0]);
                Solution::merge(&a[1..], b, seq, ans);
                seq.pop();
            }
            if !b.is_empty() {
                seq.push(b[0]);
                Solution::merge(a, &b[1..], seq, ans);
                seq.pop();
            }
        }
    }

    pub fn bst_sequences(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if let Some(node) = root {
            let arr_l = Solution::bst_sequences(node.as_ref().borrow().left.clone());
            let arr_r = Solution::bst_sequences(node.as_ref().borrow().right.clone());
            let mut ans = vec![];
            let mut seq = vec![node.as_ref().borrow().val];
            for a in &arr_l {
                for b in &arr_r {
                    Solution::merge(a, b, &mut seq, &mut ans);
                }
            }
            ans
        } else {
            vec![vec![]]
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::rctree::{TreeNode, tree};

    #[test]
    fn test1() {
        let root = tree![2, 1, 3];
        let ans = [[2, 1, 3], [2, 3, 1]];
        let ans: Vec<Vec<i32>> = ans.into_iter().map(Vec::from).collect();
        assert_eq!(Solution::bst_sequences(root), ans);
    }
    #[test]
    fn test2() {
        let root = tree![4, 1, null, null, 3, 2];
        let ans = [[4, 1, 3, 2]];
        let ans: Vec<Vec<i32>> = ans.into_iter().map(Vec::from).collect();
        assert_eq!(Solution::bst_sequences(root), ans);
    }
}
