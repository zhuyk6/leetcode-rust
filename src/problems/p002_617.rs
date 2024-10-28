pub struct Solution;

impl Solution {
    pub fn minimum_visited_cells(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut f = vec![vec![i32::MAX; n]; m];

        let mut stack_row: Vec<Vec<(i32, usize)>> = vec![vec![]; m];
        let mut stack_col: Vec<Vec<(i32, usize)>> = vec![vec![]; n];

        f[m - 1][n - 1] = 1;

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                let w = grid[i][j] as usize;
                println!("({i}, {j}):");

                // col j
                // for x in (i + 1)..=(i + w).min(m - 1) {
                //     f[i][j] = f[i][j].min(f[x][j].saturating_add(1));
                // }
                if !stack_col[j].is_empty() {
                    let mut l = 0;
                    let mut r = stack_col[j].len();
                    let mut best = i32::MAX;
                    while l < r {
                        let mid = (l + r) >> 1;
                        println!(
                            "l = {l}, r = {r}, mid = {mid}, stack = {:?}",
                            stack_col[j][mid]
                        );
                        if stack_col[j][mid].1 <= i + w {
                            best = stack_col[j][mid].0;
                            r = mid;
                        } else {
                            l = mid + 1;
                        }
                    }
                    println!("best = {best}");
                    f[i][j] = f[i][j].min(best.saturating_add(1));
                }

                // row i
                // for y in (j + 1)..=(j + w).min(n - 1) {
                //     f[i][j] = f[i][j].min(f[i][y].saturating_add(1));
                // }
                if !stack_row[i].is_empty() {
                    let mut l = 0;
                    let mut r = stack_row[i].len();
                    let mut best = i32::MAX;
                    while l < r {
                        let mid = (l + r) >> 1;
                        if stack_row[i][mid].1 <= j + w {
                            best = stack_row[i][mid].0;
                            r = mid;
                        } else {
                            l = mid + 1;
                        }
                    }
                    f[i][j] = f[i][j].min(best.saturating_add(1));
                }

                println!("f = {}", f[i][j]);

                // push into stack
                while let Some((v, _pos)) = stack_row[i].last() {
                    if *v >= f[i][j] {
                        stack_row[i].pop();
                    } else {
                        break;
                    }
                }
                stack_row[i].push((f[i][j], j));

                while let Some((v, _pos)) = stack_col[j].last() {
                    if *v >= f[i][j] {
                        stack_col[j].pop();
                    } else {
                        break;
                    }
                }
                stack_col[j].push((f[i][j], i));
            }
        }

        match f[0][0] {
            i32::MAX => -1,
            v => v,
        }
    }
}

#[test]
fn test1() {
    let grid = [[3, 4, 2, 1], [4, 2, 3, 1], [2, 1, 0, 0], [2, 4, 0, 0]];
    let grid: Vec<Vec<i32>> = grid.into_iter().map(Vec::from).collect();
    assert_eq!(Solution::minimum_visited_cells(grid), 4);
}

#[test]
fn test2() {
    let grid = [[3, 4, 2, 1], [4, 2, 1, 1], [2, 1, 1, 0], [3, 4, 1, 0]];
    let grid: Vec<Vec<i32>> = grid.into_iter().map(Vec::from).collect();
    assert_eq!(Solution::minimum_visited_cells(grid), 3);
}

#[test]
fn test3() {
    let grid = [[2, 1, 0], [1, 0, 0]];
    let grid: Vec<Vec<i32>> = grid.into_iter().map(Vec::from).collect();
    assert_eq!(Solution::minimum_visited_cells(grid), -1);
}

#[test]
fn test4() {
    let grid = [[0], [0], [2], [0]];
    let grid: Vec<Vec<i32>> = grid.into_iter().map(Vec::from).collect();
    assert_eq!(Solution::minimum_visited_cells(grid), -1);
}
