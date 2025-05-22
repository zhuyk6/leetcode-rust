use crate::rctree::TreeNode;
pub struct Solution;

use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
struct Dfs {
    mem: HashMap<(*mut TreeNode, bool, bool), u32>,
}

impl Dfs {
    fn dfs(&mut self, node: &RefCell<TreeNode>, put: bool, cover: bool) -> u32 {
        if let Some(&ans) = self.mem.get(&(node.as_ptr(), put, cover)) {
            return ans;
        }
        let ret = match (put, cover) {
            (true, _) => {
                let mut ans = 1;
                if let Some(left) = node.borrow().left.as_deref() {
                    let tmp = self.dfs(left, false, true);
                    let tmp = tmp.min(self.dfs(left, true, true));
                    ans += tmp;
                }
                if let Some(right) = node.borrow().right.as_deref() {
                    let tmp = self.dfs(right, false, true);
                    let tmp = tmp.min(self.dfs(right, true, true));
                    ans += tmp;
                }
                ans
            }
            (false, true) => {
                let mut ans = 0;
                if let Some(left) = node.borrow().left.as_deref() {
                    let tmp = self.dfs(left, false, false);
                    let tmp = tmp.min(self.dfs(left, true, false));
                    ans += tmp;
                }
                if let Some(right) = node.borrow().right.as_deref() {
                    let tmp = self.dfs(right, false, false);
                    let tmp = tmp.min(self.dfs(right, true, false));
                    ans += tmp;
                }
                ans
            }
            (false, false) => {
                match (
                    node.borrow().left.as_deref(),
                    node.borrow().right.as_deref(),
                ) {
                    (Some(left), Some(right)) => {
                        // left put
                        let ans1 = {
                            let ans = self.dfs(left, true, false);
                            let tmp = self.dfs(right, false, false);
                            let tmp = tmp.min(self.dfs(right, true, false));
                            ans.wrapping_add(tmp)
                        };

                        // right put
                        let ans2 = {
                            let ans = self.dfs(right, true, false);
                            let tmp = self.dfs(left, false, false);
                            let tmp = tmp.min(self.dfs(left, true, false));
                            ans.wrapping_add(tmp)
                        };

                        ans1.min(ans2)
                    }
                    (Some(left), None) => {
                        // left put
                        self.dfs(left, true, false)
                    }
                    (None, Some(right)) => {
                        // right put
                        self.dfs(right, true, false)
                    }
                    (None, None) => u32::MAX,
                }
            }
        };
        self.mem.insert((node.as_ptr(), put, cover), ret);
        ret
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let mut dfs = Dfs::default();
            let mut ans = dfs.dfs(&root, false, false);
            ans = ans.min(dfs.dfs(&root, true, false));

            if ans == u32::MAX {
                return -1;
            }
            ans as i32
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rctree::tree;

    #[test]
    fn sample1() {
        let root = tree![0, 0, null, 0, 0];
        assert_eq!(Solution::min_camera_cover(root), 1);
    }

    #[test]
    fn sample2() {
        let root = tree![0, 0, null, 0, null, 0, null, null, 0];
        assert_eq!(Solution::min_camera_cover(root), 2);
    }
}
