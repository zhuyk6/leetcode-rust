struct Solution;

const MOD: i32 = 1_000_000_007;

#[inline]
fn val(c: u8) -> i32 {
    (c - b'0') as i32
}

#[inline]
fn update(f: &mut (i32, i32), g: (i32, i32), d: i32) {
    match f.0.cmp(&(g.0 + d)) {
        std::cmp::Ordering::Less => *f = (g.0 + d, g.1),
        std::cmp::Ordering::Equal => f.1 = (f.1 + g.1) % MOD,
        std::cmp::Ordering::Greater => {}
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        let grid: Vec<Vec<u8>> = board.into_iter().map(|s| s.into_bytes()).collect();
        let m = grid.len();
        let n = grid[0].len();

        let mut f = vec![vec![(0, 0); n]; m];
        f[m - 1][n - 1] = (0, 1);

        for i in (0..(m - 1)).rev() {
            if grid[i][n - 1] != b'X' && f[i + 1][n - 1] != (0, 0) {
                f[i][n - 1] = (f[i + 1][n - 1].0 + val(grid[i][n - 1]), 1);
            }
        }

        for j in (0..(n - 1)).rev() {
            if grid[m - 1][j] != b'X' && f[m - 1][j + 1] != (0, 0) {
                f[m - 1][j] = (f[m - 1][j + 1].0 + val(grid[m - 1][j]), 1);
            }
        }

        for i in (0..(m - 1)).rev() {
            for j in (0..(n - 1)).rev() {
                f[i][j] = (0, 0);
                if grid[i][j] != b'X' {
                    let d = if b'0' <= grid[i][j] && grid[i][j] <= b'9' {
                        val(grid[i][j])
                    } else {
                        0
                    };
                    if f[i + 1][j] != (0, 0) {
                        let tmp = f[i + 1][j];
                        update(&mut f[i][j], tmp, d);
                    }
                    if f[i][j + 1] != (0, 0) {
                        let tmp = f[i][j + 1];
                        update(&mut f[i][j], tmp, d);
                    }
                    if f[i + 1][j + 1] != (0, 0) {
                        let tmp = f[i + 1][j + 1];
                        update(&mut f[i][j], tmp, d);
                    }
                }
                println!("f[{i}][{j}] = {:?}", f[i][j]);
            }
        }

        vec![f[0][0].0, f[0][0].1]
    }
}

#[test]
fn test1() {
    let board = vec!["E23".to_string(), "2X2".to_string(), "12S".to_string()];
    assert_eq!(Solution::paths_with_max_score(board), vec![7, 1]);
}

#[test]
fn test2() {
    let board = vec!["E12".to_string(), "1X1".to_string(), "21S".to_string()];
    assert_eq!(Solution::paths_with_max_score(board), vec![4, 2]);
}

#[test]
fn test3() {
    let board = vec!["E11".to_string(), "XXX".to_string(), "11S".to_string()];
    assert_eq!(Solution::paths_with_max_score(board), vec![0, 0]);
}
