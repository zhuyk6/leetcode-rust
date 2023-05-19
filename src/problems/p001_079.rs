use std::collections::HashSet;

struct Solver {
    n: usize,
    chars: Vec<char>,
    pre: String,
    vis: Vec<bool>,
    ans: HashSet<String>,
}

impl Solver {
    pub fn new(chars: String) -> Self {
        let chars: Vec<char> = chars.chars().collect();
        let n = chars.len();
        Solver {
            n,
            chars,
            pre: String::with_capacity(n),
            vis: vec![false; n],
            ans: HashSet::new(),
        }
    }

    pub fn dfs(&mut self, x: usize) {
        if x > 0 {
            self.ans.insert(self.pre.clone());
        }
        if x >= self.n { return; }
        for i in 0..self.n {
            if !self.vis[i] {
                self.vis[i] = true;
                self.pre.push(self.chars[i]);
                self.dfs(x + 1);
                self.pre.pop();
                self.vis[i] = false;
            }
        }
    }
}

pub fn num_tile_possibilities(tiles: String) -> i32 {
    let mut sol = Solver::new(tiles);
    sol.dfs(0);
    sol.ans.len() as i32
}

#[test]
fn example() {
    let s = "AAB".to_string();
    assert_eq!(num_tile_possibilities(s), 8);

    let s = "AAABBC".to_string();
    assert_eq!(num_tile_possibilities(s), 188);

    let s = "V".to_string();
    assert_eq!(num_tile_possibilities(s), 1);
}
