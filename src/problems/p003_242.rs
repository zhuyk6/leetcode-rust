pub struct NeighborSum {
    grid: Vec<Vec<i32>>,
    pos: Vec<(usize, usize)>,
}

impl NeighborSum {
    pub fn new(grid: Vec<Vec<i32>>) -> Self {
        let n = grid.len();
        let mut pos = vec![(0, 0); n * n];
        for (i, row) in grid.iter().enumerate() {
            for (j, v) in row.iter().enumerate() {
                pos[*v as usize] = (i, j);
            }
        }
        Self { grid, pos }
    }

    pub fn adjacent_sum(&self, value: i32) -> i32 {
        let (i, j) = self.pos[value as usize];
        let mut acc = 0;
        if i > 0 {
            acc += self.grid[i - 1][j];
        }
        if j > 0 {
            acc += self.grid[i][j - 1];
        }
        let n = self.grid.len();
        if i + 1 < n {
            acc += self.grid[i + 1][j];
        }
        if j + 1 < n {
            acc += self.grid[i][j + 1];
        }
        acc
    }

    pub fn diagonal_sum(&self, value: i32) -> i32 {
        let (i, j) = self.pos[value as usize];
        let n = self.grid.len();
        let mut acc = 0;
        if i > 0 && j > 0 {
            acc += self.grid[i - 1][j - 1];
        }
        if i > 0 && j + 1 < n {
            acc += self.grid[i - 1][j + 1];
        }
        if i + 1 < n && j > 0 {
            acc += self.grid[i + 1][j - 1];
        }
        if i + 1 < n && j + 1 < n {
            acc += self.grid[i + 1][j + 1];
        }
        acc
    }
}
