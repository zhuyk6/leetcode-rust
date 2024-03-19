struct Solution;

struct JoinSet {
    fa: Vec<(usize, usize)>,
}

impl JoinSet {
    fn new(n: usize) -> Self {
        JoinSet {
            fa: (0..n).zip(std::iter::repeat(1)).collect(),
        }
    }

    fn get_fa(&mut self, x: usize) -> usize {
        if self.fa[x].0 != x {
            self.fa[x].0 = self.get_fa(self.fa[x].0);
        }
        self.fa[x].0
    }
}

#[allow(unused)]
impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut joinset = JoinSet::new(n);

        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            let fx = joinset.get_fa(x);
            let fy = joinset.get_fa(y);
            if fx != fy {
                joinset.fa[fx].0 = fy;
                joinset.fa[fy].1 += joinset.fa[fx].1;
            }
        }

        let mut vis = vec![false; n];
        let mut ans = 0;
        for i in 0..n {
            let fi = joinset.get_fa(i);
            if !vis[fi] {
                vis[fi] = true;
                let s = joinset.fa[fi].1;
                println!("fi = {fi}, s = {s}");
                ans += s * (n - s);
            }
        }
        (ans / 2) as i64
    }
}

#[test]
fn test1() {
    let n = 3;
    let edges = [[0, 1], [0, 2], [1, 2]];
    let edges: Vec<Vec<i32>> = edges.into_iter().map(Vec::from).collect();

    assert_eq!(Solution::count_pairs(n, edges), 0);
}

#[test]
fn test2() {
    let n = 7;
    let edges = [[0, 2], [0, 5], [2, 4], [1, 6], [5, 4]];
    let edges: Vec<Vec<i32>> = edges.into_iter().map(Vec::from).collect();

    assert_eq!(Solution::count_pairs(n, edges), 14);
}
