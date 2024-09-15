struct Solution;

struct Node {
    left: usize,
    right: usize,
    min: usize,
    lc: Option<Box<Node>>,
    rc: Option<Box<Node>>,
}

struct SegmentTree {
    root: Box<Node>,
}

impl SegmentTree {
    fn build(l: usize, r: usize) -> Self {
        fn _build(l: usize, r: usize) -> Box<Node> {
            let mut node = Node {
                left: l,
                right: r,
                min: usize::MAX,
                lc: None,
                rc: None,
            };

            if l < r {
                let mid = (l + r) >> 1;
                let lc = _build(l, mid);
                let rc = _build(mid + 1, r);
                node.lc = Some(lc);
                node.rc = Some(rc);
            }
            Box::new(node)
        }

        Self { root: _build(l, r) }
    }

    fn insert(&mut self, pos: usize, val: usize) {
        fn _insert(node: &mut Node, pos: usize, val: usize) {
            if node.left == node.right {
                node.min = node.min.min(val);
                return;
            }
            let mid = (node.left + node.right) >> 1;
            if pos <= mid {
                _insert(node.lc.as_deref_mut().unwrap(), pos, val);
            } else {
                _insert(node.rc.as_deref_mut().unwrap(), pos, val);
            }
            node.min = node
                .lc
                .as_ref()
                .map(|n| n.min)
                .min(node.rc.as_ref().map(|n| n.min))
                .unwrap();
        }
        _insert(&mut self.root, pos, val);
    }

    fn query(&self, l: usize, r: usize) -> usize {
        fn _query(node: &Node, l: usize, r: usize) -> usize {
            if l <= node.left && node.right <= r {
                return node.min;
            }
            let mid = (node.left + node.right) >> 1;
            let mut ans = usize::MAX;
            if l <= mid {
                ans = ans.min(_query(node.lc.as_deref().unwrap(), l, r));
            }
            if mid < r {
                ans = ans.min(_query(node.rc.as_ref().unwrap(), l, r));
            }
            ans
        }
        _query(&self.root, l, r)
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn leftmost_building_queries(heights: Vec<i32>, mut queries: Vec<Vec<i32>>) -> Vec<i32> {
        // discretize `heights`
        let n = heights.len();
        let (h, tot) = {
            let mut h = vec![0usize; n];
            let mut idx: Vec<usize> = (0..n).collect();
            idx.sort_by_key(|&i| heights[i]);
            let mut tot = 0;
            let mut pre = i32::MIN;
            for i in 0..n {
                if heights[idx[i]] > pre {
                    tot += 1;
                }
                h[idx[i]] = tot;
                pre = heights[idx[i]];
            }
            println!("h = {h:?}");

            (h, tot)
        };

        // make sure q[0] < q[1]
        for query in &mut queries {
            let i = i32::min(query[0], query[1]);
            let j = i32::max(query[0], query[1]);
            query[0] = i;
            query[1] = j;
        }
        // sort queries by Bob
        let mut query_indices: Vec<usize> = (0..queries.len()).collect();
        query_indices.sort_by_key(|&i| queries[i][1]);

        let mut answer = vec![-1; queries.len()];

        let mut cur_pos = n;
        let mut tree = SegmentTree::build(0, tot + 1);

        for &query_idx in query_indices.iter().rev() {
            let i = queries[query_idx][0] as usize;
            let j = queries[query_idx][1] as usize;

            println!("query ({i}, {j})");

            if i == j || h[j] > h[i] {
                answer[query_idx] = j as i32;
            } else {
                while cur_pos > j + 1 {
                    cur_pos -= 1;
                    // add h[cur_pos] to data structure
                    tree.insert(h[cur_pos], cur_pos);
                }
                // query [j+1, n)
                match tree.query(h[i] + 1, tot) {
                    usize::MAX => {
                        answer[query_idx] = -1;
                    }
                    p => {
                        answer[query_idx] = p as i32;
                    }
                }
            }
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn sample1() {
        let heights = vec![6, 4, 8, 5, 2, 7];
        let queries = array![[0, 1], [0, 3], [2, 4], [3, 4], [2, 2]];
        let queries: Vec<Vec<_>> = queries.outer_iter().map(|row| row.to_vec()).collect();
        let ans = vec![2, 5, -1, 5, 2];
        assert_eq!(Solution::leftmost_building_queries(heights, queries), ans);
    }

    #[test]
    fn sample2() {
        let heights = vec![5, 3, 8, 2, 6, 1, 4, 6];
        let queries = array![[0, 7], [3, 5], [5, 2], [3, 0], [1, 6]];
        let queries: Vec<Vec<_>> = queries.outer_iter().map(|row| row.to_vec()).collect();
        let ans = vec![7, 6, -1, 4, 6];
        assert_eq!(Solution::leftmost_building_queries(heights, queries), ans);
    }
}
