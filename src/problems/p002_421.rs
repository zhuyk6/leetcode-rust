struct Solution;

struct JoinSet {
    fa: Vec<(usize, (i32, usize))>,
}

impl JoinSet {
    fn new(n: usize, node_vals: &[i32]) -> Self {
        let fa: Vec<(usize, (i32, usize))> = (0..n).map(|i| (i, (node_vals[i], 1))).collect();
        JoinSet { fa }
    }

    fn get_fa(&mut self, x: usize) -> usize {
        if self.fa[x].0 != x {
            self.fa[x].0 = self.get_fa(self.fa[x].0);
        }
        self.fa[x].0
    }

    fn link(&mut self, x: usize, fx: usize, tag: i32) {
        self.fa[x].0 = fx;
        if self.fa[fx].1 .0 != tag {
            self.fa[fx].1 = (tag, 0);
        }
        if self.fa[x].1 .0 == tag {
            self.fa[fx].1 .1 += self.fa[x].1 .1;
        }
    }
}

#[allow(unused)]
impl Solution {
    pub fn number_of_good_paths(node_vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = node_vals.len();
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];

        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            graph[x].push(y);
            graph[y].push(x);
        }

        use std::collections::BTreeMap;
        let mut val_to_nodes: BTreeMap<i32, Vec<usize>> = Default::default();
        for (i, v) in node_vals.iter().enumerate() {
            val_to_nodes.entry(*v).or_default().push(i);
        }

        let mut join_set = JoinSet::new(n, &node_vals);
        let mut ans = n;
        for (k, v) in val_to_nodes {
            println!("k = {k}, v = {v:?}");
            for x in v {
                let mut acc = 0;
                for &y in &graph[x] {
                    // can't link nodes with higher value
                    if node_vals[y] > node_vals[x] {
                        continue;
                    }

                    let fy = join_set.get_fa(y);
                    if join_set.fa[fy].1 .0 != k {
                        join_set.fa[fy].1 = (k, 0);
                    }

                    let fx = join_set.get_fa(x);
                    println!("x = {x}, fx = {fx}, y = {y}, fy = {fy}");

                    // already in the same set
                    if fx == fy {
                        continue;
                    }
                    acc += join_set.fa[fy].1 .1 * join_set.fa[fx].1 .1;
                    join_set.link(fy, fx, k);
                }
                println!("acc = {acc}");
                ans += acc;
            }
        }

        ans as i32
    }
}

#[test]
fn test1() {
    let vals = vec![1, 3, 2, 1, 3];
    let edges = [[0, 1], [0, 2], [2, 3], [2, 4]];
    let edges: Vec<Vec<i32>> = edges.into_iter().map(Vec::from).collect();

    assert_eq!(Solution::number_of_good_paths(vals, edges), 6);
}

#[test]
fn test2() {
    let vals = vec![1, 1, 2, 2, 3];
    let edges = [[0, 1], [1, 2], [2, 3], [2, 4]];
    let edges: Vec<Vec<i32>> = edges.into_iter().map(Vec::from).collect();

    assert_eq!(Solution::number_of_good_paths(vals, edges), 7);
}

#[test]
fn test3() {
    let vals = vec![1];
    let edges: [[i32; 2]; 0] = [];
    let edges: Vec<Vec<i32>> = edges.into_iter().map(Vec::from).collect();

    assert_eq!(Solution::number_of_good_paths(vals, edges), 1);
}

#[test]
fn test4() {
    let vals = vec![2, 5, 5, 1, 5, 2, 3, 5, 1, 5];
    let edges = [
        [0, 1],
        [2, 1],
        [3, 2],
        [3, 4],
        [3, 5],
        [5, 6],
        [1, 7],
        [8, 4],
        [9, 7],
    ];
    let edges: Vec<Vec<i32>> = edges.into_iter().map(Vec::from).collect();

    assert_eq!(Solution::number_of_good_paths(vals, edges), 20);
}
