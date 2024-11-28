pub struct Solution;

struct Solver {
    row: usize,
    col: usize,
    n: usize,
    mask_s: usize,
    mask_t: usize,
    grid: Vec<Vec<i32>>,
    s: (usize, usize),
    t: (usize, usize),
}

impl Solver {
    #[inline]
    fn index(&self, x: usize, y: usize) -> usize {
        x * self.col + y
    }

    fn with_grid(grid: Vec<Vec<i32>>) -> Self {
        let row = grid.len();
        let col = grid[0].len();

        let index = |i: usize, j: usize| -> usize { i * col + j };

        let n = row * col;
        let mut mask: usize = (1 << n) - 1;

        let mut s = (0, 0);
        let mut t = (0, 0);

        #[allow(clippy::needless_range_loop)]
        for i in 0..row {
            for j in 0..col {
                match grid[i][j] {
                    1 => s = (i, j),
                    2 => t = (i, j),
                    -1 => mask ^= 1 << index(i, j),
                    _ => {}
                }
            }
        }

        Self {
            row,
            col,
            n,
            mask_t: mask,
            grid,
            s,
            t,
            mask_s: 1 << index(s.0, s.1),
        }
    }

    #[allow(dead_code)]
    fn dfs(&self, x: usize, y: usize, set: usize, mem: &mut [Vec<i32>]) -> i32 {
        let idx = self.index(x, y);
        if mem[set][idx] >= 0 {
            return mem[set][idx];
        }

        if self.s == (x, y) && set == self.mask_s {
            return 1;
        }

        let mut ans = 0;
        if x > 0 && self.grid[x - 1][y] != -1 {
            let idy = self.index(x - 1, y);
            if set & (1 << idy) > 0 {
                ans += self.dfs(x - 1, y, set ^ (1 << idx), mem);
            }
        }
        if y > 0 && self.grid[x][y - 1] != -1 {
            let idy = self.index(x, y - 1);
            if set & (1 << idy) > 0 {
                ans += self.dfs(x, y - 1, set ^ (1 << idx), mem);
            }
        }

        if x + 1 < self.row && self.grid[x + 1][y] != -1 {
            let idy = self.index(x + 1, y);
            if set & (1 << idy) > 0 {
                ans += self.dfs(x + 1, y, set ^ (1 << idx), mem);
            }
        }
        if y + 1 < self.col && self.grid[x][y + 1] != -1 {
            let idy = self.index(x, y + 1);
            if set & (1 << idy) > 0 {
                ans += self.dfs(x, y + 1, set ^ (1 << idx), mem);
            }
        }

        mem[set][idx] = ans;
        ans
    }

    fn answer(&self) -> i32 {
        // let mut mem = vec![vec![-1; self.n]; 1 << self.n];
        // self.dfs(self.t.0, self.t.1, self.mask_t, &mut mem)

        let mut dp = vec![vec![0; self.n]; 1 << self.n];
        let ids = self.index(self.s.0, self.s.1);
        dp[1 << ids][ids] = 1;

        for set in (1 << ids)..=self.mask_t {
            for i in 0..self.n {
                if dp[set][i] == 0 {
                    continue;
                }
                let (x, y) = (i / self.col, i % self.col);

                if x > 0 && self.grid[x - 1][y] != -1 {
                    let idy = self.index(x - 1, y);
                    if set & (1 << idy) == 0 {
                        dp[set ^ (1 << idy)][idy] += dp[set][i];
                    }
                }
                if y > 0 && self.grid[x][y - 1] != -1 {
                    let idy = self.index(x, y - 1);
                    if set & (1 << idy) == 0 {
                        dp[set ^ (1 << idy)][idy] += dp[set][i];
                    }
                }
                if x + 1 < self.row && self.grid[x + 1][y] != -1 {
                    let idy = self.index(x + 1, y);
                    if set & (1 << idy) == 0 {
                        dp[set ^ (1 << idy)][idy] += dp[set][i];
                    }
                }
                if y + 1 < self.col && self.grid[x][y + 1] != -1 {
                    let idy = self.index(x, y + 1);
                    if set & (1 << idy) == 0 {
                        dp[set ^ (1 << idy)][idy] += dp[set][i];
                    }
                }
            }
        }

        let idt = self.index(self.t.0, self.t.1);
        dp[self.mask_t][idt]
    }
}

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let sol = Solver::with_grid(grid);
        sol.answer()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let grid = nested_vec![[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 2, -1]];
        assert_eq!(Solution::unique_paths_iii(grid), 2);
    }

    #[test]
    fn sample2() {
        let grid = nested_vec![[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 2]];
        assert_eq!(Solution::unique_paths_iii(grid), 4);
    }

    #[test]
    fn sample3() {
        let grid = nested_vec![[0, 1], [2, 0]];
        assert_eq!(Solution::unique_paths_iii(grid), 0);
    }
}
