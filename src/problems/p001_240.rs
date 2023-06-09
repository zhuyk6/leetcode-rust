
struct DFS {
    m: usize,
    n: usize,
    grid: Vec<Vec<bool>>,
}

impl DFS {
    #[allow(dead_code)]
    pub fn new(m: usize, n: usize) -> Self {
        DFS { m, n, grid: vec![vec![false; n]; m] }
    }

    #[allow(dead_code)]
    fn put(&mut self, left_top: (usize, usize), size: usize) {
        for i in left_top.0 .. (left_top.0 + size) {
            for j in left_top.1 .. (left_top.1 + size) {
                self.grid[i][j] = true;
            }
        }
    }

    #[allow(dead_code)]
    fn check_size(&self, left_top: (usize, usize), size: usize) -> bool {
        for i in left_top.0 .. (left_top.0 + size) {
            if i >= self.m { return false; }
            for j in left_top.1 .. (left_top.1 + size) {
                if j >= self.n { return false; }
                if self.grid[i][j] {
                    return false;
                }
            }
        }
        true
    }

    #[allow(dead_code)]
    fn get_size(&self, left_top: (usize, usize)) -> usize {
        let mut size = usize::MAX;
        let mut l = 1;
        while self.check_size(left_top, l) {
            size = l;
            l += 1;
        }
        size
    }

    #[allow(dead_code)]
    fn calculate(&mut self, first_size: usize) -> usize {
        for row in &mut self.grid {
            row.fill(false);
        }

        self.put((0, 0), first_size);
        let mut ans = 1;
        for i in 0..self.m {
            for j in 0..self.n {
                if !self.grid[i][j] {
                    let size = self.get_size((i, j));
                    ans += 1;
                    self.put((i, j), size);
                }
            }
        }
        ans
    }

    #[allow(dead_code)]
    pub fn solve(&mut self) -> usize {
        let max_size = self.m.min(self.n);
        let mut ans = usize::MAX;
        for size in (1..=max_size).rev() {
            let tmp = self.calculate(size);
            ans = ans.min(tmp);
        }
        ans
    }
}

#[allow(dead_code)]
pub fn tiling_rectangle(n: i32, m: i32) -> i32 {
    let mut dfs = DFS::new(n as usize, m as usize);
    dfs.solve() as i32
}

#[test]
fn example1() {
    let n = 2;
    let m = 3;
    assert_eq!(tiling_rectangle(n, m), 3);
}
#[test]
fn example2() {
    let n = 5;
    let m = 8;
    assert_eq!(tiling_rectangle(n, m), 5);
}
#[test]
fn example3() {
    let n = 11;
    let m = 13;
    assert_eq!(tiling_rectangle(n, m), 6);
}
