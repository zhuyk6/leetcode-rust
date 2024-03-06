struct Solution;

#[allow(unused, clippy::needless_range_loop)]
impl Solution {
    pub fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        for row in grid.iter_mut() {
            row.sort();
        }
        let mut ans = 0;
        let n = grid.len();
        let m = grid[0].len();
        for i in 0..m {
            let mut m = grid[0][i];
            for j in 0..n {
                m = m.max(grid[j][i]);
            }
            ans += m;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example() {
        let grid = vec![vec![1, 2, 4], vec![3, 3, 1]];
        assert_eq!(8, Solution::delete_greatest_value(grid));

        let grid = vec![vec![10]];
        assert_eq!(10, Solution::delete_greatest_value(grid));
    }
}
