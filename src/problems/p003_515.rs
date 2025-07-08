pub struct Solution;

struct Tree {
    sons: Vec<Vec<(usize, i32)>>,
}

impl Tree {
    fn new(n: usize) -> Self {
        Tree {
            sons: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, w: i32) {
        self.sons[u].push((v, w));
        self.sons[v].push((u, w));
    }

    fn dfs(
        &self,
        x: usize,
        fa: usize,
        score: &mut [i32],
        start: &mut [usize],
        end: &mut [usize],
        arr: &mut Vec<i32>,
    ) {
        start[x] = arr.len();
        arr.push(score[x]);

        for &(y, w) in &self.sons[x] {
            if y == fa {
                continue;
            }
            score[y] = w;
            self.dfs(y, x, score, start, end, arr);
        }

        end[x] = arr.len();
        arr.push(-score[x]);
    }
}

struct BinaryIndexedTree {
    n: usize,
    arr: Vec<i32>,
}

impl BinaryIndexedTree {
    #[allow(unused)]
    #[must_use]
    fn new(n: usize) -> Self {
        BinaryIndexedTree {
            n,
            arr: vec![0; n + 1],
        }
    }

    fn lowbit(x: usize) -> usize {
        x & x.wrapping_neg()
    }

    fn add(&mut self, mut i: usize, x: i32) {
        assert!(i > 0);
        while i <= self.n {
            self.arr[i] += x;
            i += Self::lowbit(i);
        }
    }

    fn sum(&self, mut i: usize) -> i32 {
        let mut s = 0;
        while i > 0 {
            s += self.arr[i];
            i -= Self::lowbit(i);
        }
        s
    }
}

impl From<Vec<i32>> for BinaryIndexedTree {
    fn from(mut arr: Vec<i32>) -> Self {
        let n = arr.len();
        for i in 0..n {
            let j = i + Self::lowbit(i);
            if j < n {
                arr[j] += arr[i];
            }
        }
        Self {
            n: n - 1, // Adjust for 1-based indexing
            arr,
        }
    }
}

impl Solution {
    pub fn tree_queries(n: i32, edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;

        let mut tree = Tree::new(n + 1);
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let w = e[2];
            tree.add_edge(u, v, w);
        }

        let mut score = vec![0; n + 1];
        let mut start = vec![0; n + 1];
        let mut end = vec![0; n + 1];
        let mut arr = vec![-1];
        tree.dfs(1, 0, &mut score, &mut start, &mut end, &mut arr);

        // println!("start: {start:?}");
        // println!("end: {end:?}");
        // println!("arr: {arr:?}");

        // let mut bit = BinaryIndexedTree::new(arr.len() - 1);
        // for (i, v) in arr.iter().enumerate().skip(1) {
        //     bit.add(i, *v);
        // }
        let mut bit = BinaryIndexedTree::from(arr);

        let mut ans = vec![];
        for query in queries {
            match query[0] {
                1 => {
                    let x: usize = {
                        let u = query[1] as usize;
                        let v = query[2] as usize;
                        if start[v] < start[u] { u } else { v }
                    };
                    let w = query[3];
                    let delta = w - score[x];
                    score[x] = w;
                    bit.add(start[x], delta);
                    bit.add(end[x], -delta);
                }
                2 => {
                    let u = query[1] as usize;
                    ans.push(bit.sum(start[u]));
                }
                _ => unreachable!(),
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let n = 2;
        let edges = nested_vec![[1, 2, 7]];
        let queries = nested_vec![[2, 2], [1, 1, 2, 4], [2, 2]];
        let ans = vec![7, 4];
        assert_eq!(Solution::tree_queries(n, edges, queries), ans);
    }

    #[test]
    fn sample2() {
        let n = 3;
        let edges = nested_vec![[1, 2, 2], [1, 3, 4]];
        let queries = nested_vec![[2, 1], [2, 3], [1, 1, 3, 7], [2, 2], [2, 3]];
        let ans = vec![0, 4, 2, 7];
        assert_eq!(Solution::tree_queries(n, edges, queries), ans);
    }

    #[test]
    fn sample3() {
        let n = 4;
        let edges = nested_vec![[1, 2, 2], [2, 3, 1], [3, 4, 5]];
        let queries = nested_vec![[2, 4], [2, 3], [1, 2, 3, 3], [2, 2], [2, 3]];
        let ans = vec![8, 3, 2, 5];
        assert_eq!(Solution::tree_queries(n, edges, queries), ans);
    }

    #[test]
    fn issue() {
        let n = 4;
        let edges = nested_vec![[1, 2, 2], [4, 2, 3], [1, 3, 4]];
        let queries = nested_vec![[2, 2], [1, 3, 1, 1], [2, 3]];
        let ans = vec![2, 1];
        assert_eq!(Solution::tree_queries(n, edges, queries), ans);
    }
}
