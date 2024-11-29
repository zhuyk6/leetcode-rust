use crate::rctree::TreeNode;
pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
type Link = Option<Rc<RefCell<TreeNode>>>;

#[allow(clippy::upper_case_acronyms)]
struct DFS<'a> {
    layers: &'a mut Vec<Vec<(i32, i32)>>,
}

impl DFS<'_> {
    pub fn dfs(&mut self, t: Link, dep: usize) -> i32 {
        t.map_or(-1, |node| {
            if self.layers.len() == dep {
                self.layers.push(vec![]);
            }
            let height = 1 + self
                .dfs(node.as_ref().borrow().left.clone(), dep + 1)
                .max(self.dfs(node.as_ref().borrow().right.clone(), dep + 1));
            self.layers[dep].push((node.as_ref().borrow().val, height));
            height
        })
    }
}

impl Solution {
    pub fn tree_queries(root: Link, queries: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let mut layers = Vec::new();
        let mut dfs = DFS {
            layers: &mut layers,
        };
        dfs.dfs(root.clone(), 0);

        println!("layers: {:#?}", layers);

        let mut answer = HashMap::new();

        for (dep, layer) in layers.into_iter().enumerate() {
            if dep == 0 {
                continue;
            }

            let (_, max_h) = *layer.iter().max_by_key(|(_, h)| *h).unwrap();
            let max_cnt = layer.iter().filter(|(_, h)| *h == max_h).count();

            let second_max_h = if max_cnt > 1 {
                max_h
            } else {
                layer
                    .iter()
                    .map(|&(_, h)| h)
                    .filter(|h| *h < max_h)
                    .max()
                    .unwrap_or(-1)
            };

            for (val, height) in layer {
                let ans = dep as i32 + if height == max_h { second_max_h } else { max_h };
                answer.insert(val, ans);
            }
        }

        queries
            .into_iter()
            .map(|val| *answer.get(&val).unwrap())
            .collect()
    }
}

#[test]
fn example() {
    use crate::rctree::tree;
    let root = tree![1, 3, 4, 2, null, 6, 5, null, null, null, null, null, 7];
    let queries = vec![4, 7, 3];
    assert_eq!(Solution::tree_queries(root, queries), vec![2, 2, 3]);
}
