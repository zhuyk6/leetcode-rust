use crate::rctree::TreeNode;
pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn f(v: i32, min: Option<i32>, max: Option<i32>) -> Option<i32> {
            if let Some(l) = min {
                if let Some(r) = max {
                    Some((v - l).abs().max((v - r).abs()))
                } else {
                    Some((v - l).abs())
                }
            } else {
                max.map(|r| (v - r).abs())
            }
        }

        fn dfs(x: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> (Option<i32>, Option<i32>) {
            x.map_or_else(
                || (None, None),
                |node| {
                    let (min1, max1) = dfs(node.as_ref().borrow().left.clone(), ans);
                    let v1 = f(node.as_ref().borrow().val, min1, max1);
                    if let Some(v1) = v1 {
                        *ans = (*ans).max(v1);
                    }

                    let (min2, max2) = dfs(node.as_ref().borrow().right.clone(), ans);
                    let v2 = f(node.as_ref().borrow().val, min2, max2);
                    if let Some(v2) = v2 {
                        *ans = (*ans).max(v2);
                    }

                    let min = node.as_ref().borrow().val;
                    let min = min1.unwrap_or(i32::MAX).min(min);
                    let min = min2.unwrap_or(i32::MAX).min(min);

                    let max = node.as_ref().borrow().val;
                    let max = max1.unwrap_or(i32::MIN).max(max);
                    let max = max2.unwrap_or(i32::MIN).max(max);

                    (Some(min), Some(max))
                },
            )
        }

        let mut ans = 0;
        dfs(root, &mut ans);
        ans
    }
}
