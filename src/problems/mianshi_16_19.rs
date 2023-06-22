struct JoinSet {
    fa: Vec<usize>,
    size: Vec<usize>,
}

impl JoinSet {
    fn new(n: usize) -> Self {
        JoinSet {
            fa: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn getfa(&mut self, x: usize) -> usize {
        if x != self.fa[x] {
            self.fa[x] = self.getfa(self.fa[x]);
        }
        self.fa[x]
    }

    fn join(&mut self, x: usize, y: usize) {
        let f1 = self.getfa(x);
        let f2 = self.getfa(y);
        if f1 != f2 {
            self.fa[f1] = f2;
            self.size[f2] += self.size[f1];
        }
    }
}

#[allow(unused)]
pub fn pond_sizes(land: Vec<Vec<i32>>) -> Vec<i32> {
    let m = land.len();
    let n = land[0].len();

    let f = |row: usize, col: usize| -> usize { row * n + col };

    let mut set = JoinSet::new(m * n);

    for i in 0..m {
        for j in 0..n {
            if land[i][j] == 0 {
                for u in i.saturating_sub(1)..=(i + 1).min(m-1) {
                    for v in j.saturating_sub(1)..=(j + 1).min(n-1) {
                        if land[u][v] == 0 {
                            // println!("({i}, {j}) -- ({u}, {v})");
                            set.join(f(i, j), f(u, v));
                        }
                    }
                }
            }
        }
    }

    let mut ans = vec![];
    for i in 0..m {
        for j in 0..n {
            if land[i][j] == 0 && set.fa[f(i, j)] == f(i, j) {
                ans.push(set.size[f(i, j)] as i32);
            }
        }
    }
    ans.sort();
    ans
}

#[test]
fn example() {
    let land = vec![
        vec![0, 2, 1, 0],
        vec![0, 1, 0, 1],
        vec![1, 1, 0, 1],
        vec![0, 1, 0, 1],
    ];
    assert_eq!(pond_sizes(land), vec![1, 2, 4]);
}
