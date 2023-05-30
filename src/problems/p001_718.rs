
struct DFS {
    n: usize,
    ans: Vec<i32>,
    used: Vec<bool>,
}

impl DFS {
    pub fn new(n: usize) -> Self {
        DFS { n, ans: vec![0; 2 * n - 1], used: vec![false; n+1] }
    }

    pub fn dfs(&mut self, i: usize) -> bool {
        println!("i = {}, ans = {:?}", i, self.ans);
        if i == self.n * 2 - 1 {
            return true;
        }
        if self.ans[i] > 0 {
            return self.dfs(i+1);
        }

        for x in (1..=self.n).rev() {
            if !self.used[x] {
                if x > 1 && (i + x >= self.n * 2 - 1 || self.ans[i+x] > 0) {
                    continue;
                }
                self.used[x] = true;
                self.ans[i] = x as i32;
                if x > 1 {
                    self.ans[i+x] = x as i32;
                }
                if self.dfs(i+1) {
                    return true;
                }
                self.used[x] = false;
                self.ans[i] = 0;
                if x > 1 {
                    self.ans[i+x] = 0;
                }
            }
        }
        false
    }
}

pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
    let mut dfs = DFS::new(n as usize);
    if dfs.dfs(0) {
        dfs.ans
    } else {
        panic!("There should exist an answer!")
    }
}


#[test]
fn example() {
    assert_eq!(construct_distanced_sequence(3), vec![3,1,2,3,2] );
    assert_eq!(construct_distanced_sequence(5), vec![5,3,1,4,3,5,2,4,2]);
    assert_eq!(construct_distanced_sequence(2), vec![2,1,2]);
}
