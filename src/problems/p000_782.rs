pub struct Solution;

impl Solution {
    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;
        let mut rows: HashSet<Vec<i32>> = HashSet::new();
        for row in &board {
            rows.insert(row.clone());
        }

        fn cal_moves(arr: &[i32]) -> Option<i32> {
            let (acc0, acc1): (i32, i32) = arr.iter().fold((0, 0), |(acc0, acc1), &v| {
                if v == 0 {
                    (acc0 + 1, acc1)
                } else {
                    (acc0, acc1 + 1)
                }
            });

            if (acc0 - acc1).abs() >= 2 {
                return None;
            }

            fn f(arr: &[i32], v: i32) -> i32 {
                let num_pos = arr
                    .iter()
                    .enumerate()
                    .filter(|&(i, &x)| i & 1 == 0 && x != v || i & 1 == 1 && x == v)
                    .count();
                assert!(num_pos & 1 == 0, "Error!");
                num_pos as i32 / 2
            }
            Some(match acc0.cmp(&acc1) {
                std::cmp::Ordering::Less => f(arr, 1),
                std::cmp::Ordering::Equal => i32::min(f(arr, 0), f(arr, 1)),
                std::cmp::Ordering::Greater => f(arr, 0),
            })
        }

        // two different rows
        if rows.len() != 2 {
            return -1;
        }

        let rows = rows.into_iter().collect::<Vec<_>>();
        // rows[0] must be negative of rows[1]
        if rows[0].iter().zip(&rows[1]).any(|(x, y)| x == y) {
            return -1;
        }

        let row_vec: Vec<i32> = board
            .iter()
            .map(|row| if row == &rows[0] { 0 } else { 1 })
            .collect();

        cal_moves(&row_vec)
            .and_then(|r| cal_moves(&rows[0]).map(|c| r + c))
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let board = nested_vec![[0, 1, 1, 0], [0, 1, 1, 0], [1, 0, 0, 1], [1, 0, 0, 1]];
        assert_eq!(Solution::moves_to_chessboard(board), 2)
    }

    #[test]
    fn sample2() {
        let board = nested_vec![[0, 1], [1, 0]];
        assert_eq!(Solution::moves_to_chessboard(board), 0)
    }

    #[test]
    fn sample3() {
        let board = nested_vec![[1, 0], [1, 0]];
        assert_eq!(Solution::moves_to_chessboard(board), -1)
    }

    #[test]
    fn issue1() {
        let board = nested_vec![[1, 0], [0, 1]];
        assert_eq!(Solution::moves_to_chessboard(board), 0)
    }

    #[test]
    fn issue2() {
        let board = nested_vec![[1, 1, 0], [0, 0, 1], [0, 0, 1]];
        assert_eq!(Solution::moves_to_chessboard(board), 2)
    }
}
