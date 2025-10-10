pub struct Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    UpLeft,
    UpRight,
    DownRight,
    DownLeft,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::UpLeft => Direction::UpRight,
            Direction::UpRight => Direction::DownRight,
            Direction::DownRight => Direction::DownLeft,
            Direction::DownLeft => Direction::UpLeft,
        }
    }
}

impl From<Direction> for (i32, i32) {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::UpLeft => (-1, -1),
            Direction::UpRight => (-1, 1),
            Direction::DownRight => (1, 1),
            Direction::DownLeft => (1, -1),
        }
    }
}

impl From<Direction> for usize {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::UpLeft => 0,
            Direction::UpRight => 1,
            Direction::DownRight => 2,
            Direction::DownLeft => 3,
        }
    }
}

impl TryFrom<usize> for Direction {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::UpLeft),
            1 => Ok(Direction::UpRight),
            2 => Ok(Direction::DownRight),
            3 => Ok(Direction::DownLeft),
            _ => Err("Invalid direction"),
        }
    }
}

struct Dfs {
    grid: Vec<Vec<i32>>,
    m: usize,
    n: usize,
    mem: Vec<Vec<[[i32; 2]; 4]>>,
}

impl Dfs {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        let m = grid.len();
        let n = grid[0].len();
        let mem = vec![vec![[[-1; 2]; 4]; n]; m];
        Self { grid, m, n, mem }
    }

    fn next_pos(&self, pos: (usize, usize), direction: Direction) -> Option<(usize, usize)> {
        let (dx, dy) = direction.into();
        let (x, y) = pos;
        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
        if nx >= 0 && nx < self.m as i32 && ny >= 0 && ny < self.n as i32 {
            Some((nx as usize, ny as usize))
        } else {
            None
        }
    }

    #[allow(clippy::collapsible_if)]
    fn dfs(&mut self, pos: (usize, usize), direction: Direction, left_turn: bool) -> i32 {
        let (x, y) = pos;
        let d: usize = direction.into();

        if self.mem[x][y][d][left_turn as usize] != -1 {
            return self.mem[x][y][d][left_turn as usize];
        }

        let mut ret = 0;
        if let Some((xx, yy)) = self.next_pos(pos, direction) {
            if self.grid[x][y] == 0 && self.grid[xx][yy] == 2
                || self.grid[x][y] == 2 && self.grid[xx][yy] == 0
            {
                ret = ret.max(self.dfs((xx, yy), direction, left_turn));
            }
        }

        if left_turn {
            let new_direction = direction.next();
            if let Some((xx, yy)) = self.next_pos(pos, new_direction) {
                if self.grid[x][y] == 0 && self.grid[xx][yy] == 2
                    || self.grid[x][y] == 2 && self.grid[xx][yy] == 0
                {
                    ret = ret.max(self.dfs((xx, yy), new_direction, false));
                }
            }
        }

        self.mem[x][y][d][left_turn as usize] = ret + 1;
        ret + 1
    }
}

impl Solution {
    #[allow(clippy::collapsible_if)]
    pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        let mut sol = Dfs::new(grid);

        let mut ret = 0;

        for i in 0..sol.m {
            for j in 0..sol.n {
                if sol.grid[i][j] == 1 {
                    let mut tmp = 0;
                    for d in 0..4usize {
                        let direction = d.try_into().unwrap();
                        if let Some((x, y)) = sol.next_pos((i, j), direction) {
                            if sol.grid[x][y] == 2 {
                                tmp = tmp.max(sol.dfs((x, y), direction, true));
                            }
                        }
                    }
                    ret = ret.max(tmp + 1);
                }
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let grid = nested_vec![
            [2, 2, 1, 2, 2],
            [2, 0, 2, 2, 0],
            [2, 0, 1, 1, 0],
            [1, 0, 2, 2, 2],
            [2, 0, 0, 2, 2]
        ];
        assert_eq!(Solution::len_of_v_diagonal(grid), 5);
    }

    #[test]
    fn sample2() {
        let grid = nested_vec![
            [2, 2, 2, 2, 2],
            [2, 0, 2, 2, 0],
            [2, 0, 1, 1, 0],
            [1, 0, 2, 2, 2],
            [2, 0, 0, 2, 2]
        ];
        assert_eq!(Solution::len_of_v_diagonal(grid), 4);
    }

    #[test]
    fn sample3() {
        let grid = nested_vec![
            [1, 2, 2, 2, 2],
            [2, 2, 2, 2, 0],
            [2, 0, 0, 0, 0],
            [0, 0, 2, 2, 2],
            [2, 0, 0, 2, 0]
        ];
        assert_eq!(Solution::len_of_v_diagonal(grid), 5);
    }

    #[test]
    fn sample4() {
        let grid = nested_vec![[1]];
        assert_eq!(Solution::len_of_v_diagonal(grid), 1);
    }

    #[test]
    fn issue() {
        let grid = nested_vec![
            [0, 2, 0, 2, 1, 2, 2, 2, 0, 2, 0],
            [0, 2, 1, 2, 0, 2, 0, 2, 0, 2, 0],
            [0, 2, 0, 2, 0, 2, 0, 2, 1, 2, 0]
        ];
        assert_eq!(Solution::len_of_v_diagonal(grid), 5);
    }
}
