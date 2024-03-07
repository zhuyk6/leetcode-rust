struct Solution;

struct Dfs<'a> {
    m: usize,
    n: usize,
    mat: &'a Vec<Vec<i32>>,
    f: Vec<Vec<i32>>,
}

#[allow(unused)]
impl<'a> Dfs<'a> {
    fn new(mat: &'a Vec<Vec<i32>>) -> Self {
        let m = mat.len();
        let n = mat[0].len();
        let f = vec![vec![-1; n]; m];
        Dfs { m, n, mat, f }
    }

    fn dfs(&mut self, x: usize, y: usize) -> i32 {
        if self.f[x][y] == -1 {
            self.f[x][y] = 1;
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let xx = x as i32 + dx;
                let yy = y as i32 + dy;
                if 0 <= xx && xx < self.m as i32 && 0 <= yy && yy < self.n as i32 {
                    let xx = xx as usize;
                    let yy = yy as usize;
                    if self.mat[xx][yy] > self.mat[x][y] {
                        self.f[x][y] = self.f[x][y].max(1 + self.dfs(xx, yy));
                    }
                }
            }
        }
        self.f[x][y]
    }

    fn answer(&mut self) -> i32 {
        let mut ans = -1;
        for i in 0..self.m {
            for j in 0..self.n {
                let tmp = self.dfs(i, j);
                println!("(i, j) = ({i}, {j}), f = {tmp}");
                ans = ans.max(tmp);
            }
        }
        ans
    }
}

#[allow(unused)]
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let mut dfs = Dfs::new(&matrix);
        dfs.answer()
    }
}

#[test]
fn test1() {
    let matrix = vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]];
    assert_eq!(Solution::longest_increasing_path(matrix), 4);
}

#[test]
fn test2() {
    let matrix = vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]];
    assert_eq!(Solution::longest_increasing_path(matrix), 4);
}

#[test]
fn test3() {
    let matrix = vec![vec![1]];
    assert_eq!(Solution::longest_increasing_path(matrix), 1);
}

#[test]
fn test4() {
    let matrix = vec![vec![1, 2]];
    assert_eq!(Solution::longest_increasing_path(matrix), 2);
}
