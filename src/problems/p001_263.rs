pub struct Solution;

use std::collections::VecDeque;

type Position = (usize, usize);

#[derive(Copy, Clone, Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

const DIRECTIONS: [Direction; 4] = [Direction::U, Direction::D, Direction::L, Direction::R];

impl Solution {
    pub fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let find_pos = |target: char| -> Position {
            for (i, row) in grid.iter().enumerate() {
                for (j, c) in row.iter().enumerate() {
                    if *c == target {
                        return (i, j);
                    }
                }
            }
            (m, n)
        };

        let pos_target = find_pos('T');
        let pos_start = find_pos('S');
        let pos_box = find_pos('B');

        // Check whether the point is out of the grid.
        let check = |x: i32, y: i32| -> bool { 0 <= x && x < m as i32 && 0 <= y && y < n as i32 };

        // Move the point in the given direction
        let move_point = |pos: Position, d: Direction| -> Option<Position> {
            let (x, y) = (pos.0 as i32, pos.1 as i32);
            let (xx, yy) = match d {
                Direction::U => (x - 1, y),
                Direction::D => (x + 1, y),
                Direction::L => (x, y - 1),
                Direction::R => (x, y + 1),
            };
            if !check(xx, yy) {
                None
            } else {
                Some((xx as usize, yy as usize))
            }
        };

        // Move the box according to the direction where person locates.
        let move_box = |pos: Position, from_direction: Direction| -> Option<Position> {
            let x = pos.0 as i32;
            let y = pos.1 as i32;
            let (xx, yy) = match from_direction {
                Direction::U => (x + 1, y),
                Direction::D => (x - 1, y),
                Direction::L => (x, y + 1),
                Direction::R => (x, y - 1),
            };
            if !check(xx, yy) {
                return None;
            }
            let (xx, yy) = (xx as usize, yy as usize);
            if grid[xx][yy] == '#' {
                return None;
            }
            Some((xx, yy))
        };

        // Get the available directions,
        // `pos_b` is the current position of box,
        // `pos_p` is the current position of person
        // from the current position, which directions can person go
        let get_available_directions = |pos_b: Position, pos_p: Position| -> Vec<Direction> {
            let mut que = VecDeque::new();
            let mut vis = vec![vec![false; n]; m];
            que.push_back(pos_p);
            vis[pos_p.0][pos_p.1] = true;
            while let Some((x, y)) = que.pop_front() {
                for d in DIRECTIONS {
                    if let Some((xx, yy)) = move_point((x, y), d)
                        && grid[xx][yy] != '#'
                        && (xx, yy) != pos_b
                        && !vis[xx][yy]
                    {
                        vis[xx][yy] = true;
                        que.push_back((xx, yy));
                    }
                }
            }
            let mut ans = vec![];
            for d in DIRECTIONS {
                if let Some((x, y)) = move_point((pos_b.0, pos_b.1), d)
                    && vis[x][y]
                {
                    ans.push(d);
                }
            }
            ans
        };

        let mut vis: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 4]; n]; m];
        // Position of box, person's direction w.r.t the box, and the steps
        let mut que: VecDeque<(Position, Direction, i32)> = VecDeque::new();
        // init
        {
            let available_directions = get_available_directions(pos_box, pos_start);
            for d in available_directions {
                vis[pos_box.0][pos_box.1][d as usize] = true;
                que.push_back((pos_box, d, 0));
            }
        }
        // BFS
        while let Some((pos, d, steps)) = que.pop_front() {
            if let Some(next_pos) = move_box(pos, d) {
                println!("Now: {pos:?}, d: {d:?}, next: {next_pos:?}, step: {steps}");

                if next_pos == pos_target {
                    return steps + 1;
                }
                let available_directions = get_available_directions(next_pos, pos);
                for d in available_directions {
                    if !vis[next_pos.0][next_pos.1][d as usize] {
                        vis[next_pos.0][next_pos.1][d as usize] = true;
                        que.push_back((next_pos, d, steps + 1));
                    }
                }
            }
        }
        -1
    }
}

#[test]
fn test1() {
    let grid = [
        ["#", "#", "#", "#", "#", "#"],
        ["#", "T", "#", "#", "#", "#"],
        ["#", ".", ".", "B", ".", "#"],
        ["#", ".", "#", "#", ".", "#"],
        ["#", ".", ".", ".", "S", "#"],
        ["#", "#", "#", "#", "#", "#"],
    ];
    let grid: Vec<Vec<char>> = grid
        .into_iter()
        .map(|row| row.into_iter().map(|s| s.chars().next().unwrap()).collect())
        .collect();
    assert_eq!(Solution::min_push_box(grid), 3);
}

#[test]
fn test2() {
    let grid = [
        ["#", "#", "#", "#", "#", "#"],
        ["#", "T", "#", "#", "#", "#"],
        ["#", ".", ".", "B", ".", "#"],
        ["#", "#", "#", "#", ".", "#"],
        ["#", ".", ".", ".", "S", "#"],
        ["#", "#", "#", "#", "#", "#"],
    ];
    let grid: Vec<Vec<char>> = grid
        .into_iter()
        .map(|row| row.into_iter().map(|s| s.chars().next().unwrap()).collect())
        .collect();
    assert_eq!(Solution::min_push_box(grid), -1);
}

#[test]
fn test3() {
    let grid = [
        ["#", "#", "#", "#", "#", "#"],
        ["#", "T", ".", ".", "#", "#"],
        ["#", ".", "#", "B", ".", "#"],
        ["#", ".", ".", ".", ".", "#"],
        ["#", ".", ".", ".", "S", "#"],
        ["#", "#", "#", "#", "#", "#"],
    ];
    let grid: Vec<Vec<char>> = grid
        .into_iter()
        .map(|row| row.into_iter().map(|s| s.chars().next().unwrap()).collect())
        .collect();
    assert_eq!(Solution::min_push_box(grid), 5);
}

#[test]
fn test4() {
    let grid = [
        ["#", ".", ".", "#", "#", "#", "#", "#"],
        ["#", ".", ".", "T", "#", ".", ".", "#"],
        ["#", ".", ".", ".", "#", "B", ".", "#"],
        ["#", ".", ".", ".", ".", ".", ".", "#"],
        ["#", ".", ".", ".", "#", ".", "S", "#"],
        ["#", ".", ".", "#", "#", "#", "#", "#"],
    ];
    let grid: Vec<Vec<char>> = grid
        .into_iter()
        .map(|row| row.into_iter().map(|s| s.chars().next().unwrap()).collect())
        .collect();
    assert_eq!(Solution::min_push_box(grid), 7);
}
