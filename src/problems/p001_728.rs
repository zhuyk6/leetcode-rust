// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
// enum State {
//     Finished(usize), // 0 for mouse win, 1 for cat win
//     Working,
//     Waiting,
// }

struct Solver {
    m: usize,
    n: usize,
    grid: Vec<Vec<bool>>,
    food: (usize, usize),
    jump_len: [usize; 2],
    f: Vec<Vec<Vec<Vec<Vec<usize>>>>>,
}

type Point = (usize, usize);

impl Solver {
    pub fn new(grid: Vec<String>, mouse_jump: usize, cat_jump: usize) -> (Self, Point, Point) {
        let m = grid.len();
        let n = grid[0].len();
        let mut mat = vec![vec![false; n]; m];
        let mut mouse = (0, 0);
        let mut cat = (0, 0);
        let mut food = (0, 0);
        for (i, row) in grid.into_iter().enumerate() {
            for (j, c) in row.into_bytes().into_iter().enumerate() {
                match c {
                    b'M' => {
                        mouse = (i, j);
                        mat[i][j] = true;
                    }
                    b'C' => {
                        cat = (i, j);
                        mat[i][j] = true;
                    }
                    b'F' => {
                        food = (i, j);
                        mat[i][j] = true;
                    }
                    b'.' => mat[i][j] = true,
                    b'#' => mat[i][j] = false,
                    _ => panic!("invalid char"),
                }
            }
        }

        let sol = Solver {
            m,
            n,
            grid: mat,
            food,
            jump_len: [mouse_jump, cat_jump],
            f: vec![vec![vec![vec![vec![2; n]; m]; n]; m]; 2],
        };
        (sol, mouse, cat)
    }

    fn moves(&self, turn: usize, pos: [Point; 2]) -> Vec<[Point; 2]> {
        let jump = self.jump_len[turn];
        let mut ans = vec![];
        // up
        for i in (pos[turn].0.saturating_sub(jump)..pos[turn].0).rev() {
            if !self.grid[i][pos[turn].1] {
                break;
            }
            let mut next = pos;
            next[turn].0 = i;

            ans.push(next);
        }
        // down
        for i in (pos[turn].0 + 1)..=(self.m - 1).min(pos[turn].0 + jump) {
            if !self.grid[i][pos[turn].1] {
                break;
            }
            let mut next = pos;
            next[turn].0 = i;

            ans.push(next);
        }
        // left
        for j in (pos[turn].1.saturating_sub(jump)..pos[turn].1).rev() {
            if !self.grid[pos[turn].0][j] {
                break;
            }
            let mut next = pos;
            next[turn].1 = j;

            ans.push(next);
        }
        // right
        for j in (pos[turn].1 + 1)..=(self.n - 1).min(pos[turn].1 + jump) {
            if !self.grid[pos[turn].0][j] {
                break;
            }
            let mut next = pos;
            next[turn].1 = j;

            ans.push(next);
        }
        if turn == 1 {
            ans.push(pos);
        }
        ans
    }    

    pub fn topsort(&mut self, start_pos: [Point; 2]) -> bool {
        use std::collections::VecDeque;
        
        let (m, n) = (self.m, self.n);

        // (turn, pos, dep)
        let mut que: VecDeque<(usize, [Point; 2], usize)> = VecDeque::new();

        // corner cases
        for i in 0..self.m {
            for j in 0..self.n {
                if self.grid[i][j] {
                    // cat catch mouse
                    que.push_back((0, [(i, j), (i, j)], 0));
                    que.push_back((1, [(i, j), (i, j)], 0));
                    self.f[0][i][j][i][j] = 1;
                    self.f[1][i][j][i][j] = 1;

                    if (i, j) != self.food {
                        // mouse get food
                        que.push_back((1, [self.food, (i, j)], 0));
                        self.f[1][self.food.0][self.food.1][i][j] = 0;
                        // cat get food
                        que.push_back((0, [(i, j), self.food], 0));
                        self.f[0][i][j][self.food.0][self.food.1] = 1;
                    }
                }
            }
        }

        // count degree
        let mut deg = vec![vec![vec![vec![vec![0; n]; m]; n]; m]; 2];
        for i in 0..m {
            for j in 0..n {
                if !self.grid[i][j] { continue; }
                for x in 0..m {
                    for y in 0..n {
                        if !self.grid[x][y] { continue; }
                        
                        let last_positions = self.moves(0, [(i, j), (x, y)]);
                        for pos in last_positions {
                            deg[0][pos[0].0][pos[0].1][pos[1].0][pos[1].1] += 1;
                        }

                        let last_positions = self.moves(1, [(i, j), (x, y)]);
                        for pos in last_positions {
                            deg[1][pos[0].0][pos[0].1][pos[1].0][pos[1].1] += 1;
                        }
                    }
                }
            }
        }

        while let Some((turn, now, dep)) = que.pop_front() {
            let winner = self.f[turn][now[0].0][now[0].1][now[1].0][now[1].1];
            println!("turn: {}, now: {:?}, dep: {}, winner: {}", turn, now, dep, winner);
            
            if turn == 0 && now == start_pos {
                return dep < 1000 && winner == 0;
            }

            let last_positions = self.moves(turn^1, now);

            for pos in last_positions {
                // already have winner
                if self.f[turn^1][pos[0].0][pos[0].1][pos[1].0][pos[1].1] < 2 {
                    continue;
                }
                // last can win
                if winner == turn ^ 1 {
                    self.f[turn^1][pos[0].0][pos[0].1][pos[1].0][pos[1].1] = turn ^ 1;
                } else {
                    deg[turn^1][pos[0].0][pos[0].1][pos[1].0][pos[1].1] -= 1;
                    // if all loss, then loss
                    if deg[turn^1][pos[0].0][pos[0].1][pos[1].0][pos[1].1] == 0 {
                        self.f[turn^1][pos[0].0][pos[0].1][pos[1].0][pos[1].1] = turn;
                    }
                }

                if self.f[turn^1][pos[0].0][pos[0].1][pos[1].0][pos[1].1] < 2 {
                    que.push_back((turn^1, pos, dep+1));
                }
            }
        }

        false
    }
}

#[allow(dead_code)]
pub fn can_mouse_win(grid: Vec<String>, cat_jump: i32, mouse_jump: i32) -> bool {
    let (mut sol, p1, p2) = Solver::new(grid, mouse_jump as usize, cat_jump as usize);
    // sol.dfs(0, 0, [p1, p2]) == 0
    
    sol.topsort([p1, p2])
}

#[test]
fn example1() {
    let grid = ["####F", "#C...", "M...."];
    let grid = grid.into_iter().map(|s| s.to_string()).collect();
    let cat_jump = 1;
    let mouse_jump = 2;
    assert!(can_mouse_win(grid, cat_jump, mouse_jump));
}

#[test]
fn example2() {
    let grid = ["M.C...F"];
    let grid = grid.into_iter().map(|s| s.to_string()).collect();
    let cat_jump = 1;
    let mouse_jump = 4;
    assert!(can_mouse_win(grid, cat_jump, mouse_jump));
}

#[test]
fn example3() {
    let grid = ["M.C...F"];
    let grid = grid.into_iter().map(|s| s.to_string()).collect();
    let cat_jump = 1;
    let mouse_jump = 3;
    assert!(!can_mouse_win(grid, cat_jump, mouse_jump));
}

#[test]
fn example4() {
    let grid = ["C...#", "...#F", "....#", "M...."];
    let grid = grid.into_iter().map(|s| s.to_string()).collect();
    let cat_jump = 2;
    let mouse_jump = 5;
    assert!(!can_mouse_win(grid, cat_jump, mouse_jump));
}

#[test]
fn example5() {
    let grid = [".M...", "..#..", "#..#.", "C#.#.", "...#F"];
    let grid = grid.into_iter().map(|s| s.to_string()).collect();
    let cat_jump = 3;
    let mouse_jump = 1;
    assert!(can_mouse_win(grid, cat_jump, mouse_jump));
}

#[test]
fn wrong() {
    let grid = ["CM......", "#######.", "........", "F#######"];
    let grid = grid.into_iter().map(|s| s.to_string()).collect();
    let cat_jump = 1;
    let mouse_jump = 1;
    let ans = can_mouse_win(grid, cat_jump, mouse_jump);
    assert!(ans);
}
