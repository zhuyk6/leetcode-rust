struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut ans = 0;

        for (i, row) in board.iter().enumerate() {
            for (j, &element) in row.iter().enumerate() {
                if element == 'X'
                    && (i == 0 || board[i - 1][j] == '.')
                    && (j == 0 || board[i][j - 1] == '.')
                {
                    ans += 1;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn sample1() {
        let arr = array![
            ["X", ".", ".", "X"],
            [".", ".", ".", "X"],
            [".", ".", ".", "X"]
        ];
        let board: Vec<Vec<char>> = arr
            .outer_iter()
            .map(|row| row.iter().map(|s| s.chars().next().unwrap()).collect())
            .collect();
        assert_eq!(Solution::count_battleships(board), 2);
    }

    #[test]
    fn sample2() {
        let arr = array![
            ["."]
        ];
        let board: Vec<Vec<char>> = arr
            .outer_iter()
            .map(|row| row.iter().map(|s| s.chars().next().unwrap()).collect())
            .collect();
        assert_eq!(Solution::count_battleships(board), 0);
    }
}
