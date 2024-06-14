struct Solution;

use std::collections::HashMap;

#[derive(Default, Debug)]
struct Node {
    sons: [Option<Box<Node>>; 26],
    idx: Option<usize>,
}

impl Node {
    fn insert(root: &mut Node, s: &str, idx: usize) {
        let mut now = root;
        for c in s.bytes() {
            let c = (c - b'a') as usize;
            if now.sons[c].is_none() {
                now.sons[c] = Some(Box::default());
            }
            now = now.sons[c].as_deref_mut().unwrap();
        }
        now.idx = Some(idx);
    }
}

struct TransformGraph {
    str_to_idx: HashMap<String, usize>,
    mat: Vec<Vec<i64>>,
    root: Node,
}

impl TransformGraph {
    fn new(original: &[String], changed: &[String], cost: &[i32]) -> Self {
        let mut str_to_idx = HashMap::<String, usize>::new();

        let mut tot = 0;
        let mut root = Node::default();
        let mut edges = Vec::<(usize, usize, i32)>::new();
        for ((s, t), c) in original.iter().zip(changed).zip(cost) {
            let idx_s = *str_to_idx.entry(s.clone()).or_insert_with(|| {
                let idx = tot;
                Node::insert(&mut root, s, idx);
                tot += 1;
                idx
            });
            let idx_t = *str_to_idx.entry(t.clone()).or_insert_with(|| {
                let idx = tot;
                Node::insert(&mut root, t, idx);
                tot += 1;
                idx
            });
            edges.push((idx_s, idx_t, *c));
        }

        let mut mat = vec![vec![i64::MAX; tot]; tot];

        #[allow(clippy::needless_range_loop)]
        for i in 0..tot {
            mat[i][i] = 0;
        }
        for (s, t, c) in edges {
            mat[s][t] = mat[s][t].min(c as i64);
        }

        for k in 0..tot {
            for i in (0..tot).filter(|&i| i != k) {
                for j in (0..tot).filter(|&j| j != k && j != i) {
                    mat[i][j] = mat[i][j].min(mat[i][k].saturating_add(mat[k][j]));
                }
            }
        }

        TransformGraph {
            str_to_idx,
            mat,
            root,
        }
    }

    #[allow(unused)]
    fn get_cost(&self, s: &str, t: &str) -> i64 {
        if s == t {
            0
        } else {
            self.str_to_idx
                .get(s)
                .and_then(|idx_s| self.str_to_idx.get(t).map(|idx_t| self.mat[*idx_s][*idx_t]))
                .unwrap_or(i64::MAX)
        }
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<String>,
        changed: Vec<String>,
        cost: Vec<i32>,
    ) -> i64 {
        let transform_graph = TransformGraph::new(&original, &changed, &cost);
        let n = source.len();

        let mut f = vec![i64::MAX; n];

        for i in 0..n {
            let pre = match i {
                0 => 0,
                _ => f[i - 1],
            };
            if pre == i64::MAX {
                continue;
            }
            // eprintln!("[0, {i}) : {pre}");

            // for j in i..n {
            //     let s = &source[i..=j];
            //     let t = &target[i..=j];
            //     let c = transform_graph.get_cost(s, t);
            //     f[j] = f[j].min(pre.saturating_add(c));
            //     eprintln!("j = {j}, s = {s:?}, t = {t:?}, c = {c}");
            // }
            let mut j = i;
            let mut x = Some(&transform_graph.root);
            let mut y = Some(&transform_graph.root);
            let mut is_slice_equal = true;
            while j < n {
                if source[j..j + 1] != target[j..j + 1] {
                    is_slice_equal = false;
                }
                let ch = source[j..j + 1].bytes().next().unwrap();
                let ch = (ch - b'a') as usize;
                x = x.and_then(|node| node.sons[ch].as_deref());

                let ch = target[j..j + 1].bytes().next().unwrap();
                let ch = (ch - b'a') as usize;
                y = y.and_then(|node| node.sons[ch].as_deref());

                // println!(
                //     "s = {:?}, t = {:?}, equal = {is_slice_equal}",
                //     &source[i..=j],
                //     &target[i..=j]
                // );

                if is_slice_equal {
                    f[j] = f[j].min(pre);
                } else if x.is_some() && y.is_some() {
                    let c = x
                        .and_then(|node| node.idx.as_ref())
                        .and_then(|idx_s| {
                            y.and_then(|node| node.idx.as_ref())
                                .map(|idx_t| transform_graph.mat[*idx_s][*idx_t])
                        })
                        .unwrap_or(i64::MAX);
                    f[j] = f[j].min(pre.saturating_add(c));
                } else {
                    break;
                }

                j += 1;
            }
        }

        match f[n - 1] {
            i64::MAX => -1,
            v => v,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let source = "abcd".to_string();
        let target = "acbe".to_string();
        let original = ["a", "b", "c", "c", "e", "d"];
        let changed = ["b", "c", "b", "e", "b", "e"];
        let cost = vec![2, 5, 5, 1, 2, 20];

        let original: Vec<String> = original.into_iter().map(|s| s.to_string()).collect();
        let changed: Vec<String> = changed.into_iter().map(|s| s.to_string()).collect();

        assert_eq!(
            Solution::minimum_cost(source, target, original, changed, cost),
            28
        );
    }

    #[test]
    fn sample2() {
        let source = "abcdefgh".to_string();
        let target = "acdeeghh".to_string();
        let original = ["bcd", "fgh", "thh"];
        let changed = ["cde", "thh", "ghh"];
        let cost = vec![1, 3, 5];

        let original: Vec<String> = original.into_iter().map(|s| s.to_string()).collect();
        let changed: Vec<String> = changed.into_iter().map(|s| s.to_string()).collect();

        assert_eq!(
            Solution::minimum_cost(source, target, original, changed, cost),
            9
        );
    }

    #[test]
    fn sample3() {
        let source = "abcdefgh".to_string();
        let target = "addddddd".to_string();
        let original = ["bcd", "defgh"];
        let changed = ["ddd", "ddddd"];
        let cost = vec![100, 1578];

        let original: Vec<String> = original.into_iter().map(|s| s.to_string()).collect();
        let changed: Vec<String> = changed.into_iter().map(|s| s.to_string()).collect();

        assert_eq!(
            Solution::minimum_cost(source, target, original, changed, cost),
            -1
        );
    }
}
