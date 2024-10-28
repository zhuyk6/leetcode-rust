pub struct Solution;

#[derive(Debug)]
enum Direction {
    U,
    D,
    L,
    R,
    F,
}

struct NextStep {
    m: usize,
    n: usize,
    x: usize,
    y: usize,
    cur: Direction,
}

impl NextStep {
    fn new(m: usize, n: usize, x: usize, y: usize) -> Self {
        NextStep {
            m,
            n,
            x,
            y,
            cur: Direction::U,
        }
    }
}

impl Iterator for NextStep {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.cur {
            Direction::U => {
                self.cur = Direction::D;
                if self.x > 0 {
                    Some((self.x - 1, self.y))
                } else {
                    self.next()
                }
            }
            Direction::D => {
                self.cur = Direction::L;
                if self.x + 1 < self.m {
                    Some((self.x + 1, self.y))
                } else {
                    self.next()
                }
            }
            Direction::L => {
                self.cur = Direction::R;
                if self.y > 0 {
                    Some((self.x, self.y - 1))
                } else {
                    self.next()
                }
            }
            Direction::R => {
                self.cur = Direction::F;
                if self.y + 1 < self.n {
                    Some((self.x, self.y + 1))
                } else {
                    self.next()
                }
            }
            Direction::F => None,
        }
    }
}

impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let k = (k as usize).min(m + n - 3);

        use std::collections::VecDeque;
        let mut que = VecDeque::new();
        let mut vis = vec![vec![vec![false; k + 1]; n]; m];

        if grid[0][0] == 0 {
            que.push_back(((0, 0), 0, 0));
            vis[0][0][0] = true;
        } else {
            que.push_back(((0, 0), 1, 0));
            vis[0][0][1] = true;
        }

        while let Some(((x, y), kk, step)) = que.pop_front() {
            if (x, y) == (m - 1, n - 1) {
                return step;
            }
            for (xx, yy) in NextStep::new(m, n, x, y) {
                if !vis[xx][yy][kk] {
                    if grid[xx][yy] == 0 {
                        vis[xx][yy][kk] = true;
                        que.push_back(((xx, yy), kk, step + 1));
                    } else if kk < k {
                        vis[xx][yy][kk + 1] = true;
                        que.push_back(((xx, yy), kk + 1, step + 1));
                    }
                }
            }
        }
        -1
    }
}

#[test]
fn test1() {
    let grid = [[0, 0, 0], [1, 1, 0], [0, 0, 0], [0, 1, 1], [0, 0, 0]];
    let k = 1;
    let grid: Vec<Vec<i32>> = grid.into_iter().map(Vec::from).collect();
    assert_eq!(Solution::shortest_path(grid, k), 6);
}

#[test]
fn test2() {
    let grid = [[0, 1, 1], [1, 1, 1], [1, 0, 0]];
    let k = 1;
    let grid: Vec<Vec<i32>> = grid.into_iter().map(Vec::from).collect();
    assert_eq!(Solution::shortest_path(grid, k), -1);
}

#[test]
fn mem() {
    let grid = [
        [0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1],
        [0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1],
        [1, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0],
        [1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1],
        [1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1],
        [0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1],
        [0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1],
        [1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0],
        [0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0],
        [0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0],
        [0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0],
    ];
    let k = 27;
    let grid: Vec<Vec<i32>> = grid.into_iter().map(Vec::from).collect();
    assert_eq!(Solution::shortest_path(grid, k), 24);
}
