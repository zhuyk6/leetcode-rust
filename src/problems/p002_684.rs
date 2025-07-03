pub struct Solution;

#[allow(unused, clippy::needless_range_loop)]
impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut f = vec![vec![0; n]; m];

        for j in (0..(n - 1)).rev() {
            for i in 0..m {
                for k in i.saturating_sub(1)..=(i + 1).min(m - 1) {
                    if grid[i][j] < grid[k][j + 1] {
                        f[i][j] = f[i][j].max(1 + f[k][j + 1]);
                    }
                }
            }
        }
        println!("{f:?}");

        (0..m).map(|i| f[i][0]).max().unwrap()
    }
}

#[test]
fn test1() {
    let grid = [[2, 4, 3, 5], [5, 4, 9, 3], [3, 4, 2, 11], [10, 9, 13, 15]];
    let grid: Vec<Vec<i32>> = grid.into_iter().map(Vec::from).collect();

    assert_eq!(Solution::max_moves(grid), 3);
}

#[test]
fn test2() {
    let grid = [[3, 2, 4], [2, 1, 9], [1, 1, 7]];
    let grid: Vec<Vec<i32>> = grid.into_iter().map(Vec::from).collect();

    assert_eq!(Solution::max_moves(grid), 0);
}
