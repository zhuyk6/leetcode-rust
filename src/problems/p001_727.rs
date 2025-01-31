pub struct Solution;

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut up_sum = vec![vec![0u32; n]; m];
        for j in 0..n {
            up_sum[0][j] = matrix[0][j] as u32;
        }
        for i in 1..m {
            for j in 0..n {
                if matrix[i][j] == 1 {
                    up_sum[i][j] = up_sum[i - 1][j] + 1;
                }
            }
        }

        let mut ans = 0;
        for row in &mut up_sum {
            row.sort_unstable();
            for (&h, w) in row.iter().rev().zip(1i32..) {
                ans = ans.max(w * h as i32);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nested_vec;

    #[test]
    fn sample1() {
        let matrix = nested_vec![[0, 0, 1], [1, 1, 1], [1, 0, 1]];
        assert_eq!(Solution::largest_submatrix(matrix), 4);
    }

    #[test]
    fn sample2() {
        let matrix = nested_vec![[1, 0, 1, 0, 1]];
        assert_eq!(Solution::largest_submatrix(matrix), 3);
    }

    #[test]
    fn sample3() {
        let matrix = nested_vec![[1, 1, 0], [1, 0, 1]];
        assert_eq!(Solution::largest_submatrix(matrix), 2);
    }

    #[test]
    fn sample4() {
        let matrix = nested_vec![[0, 0], [0, 0]];
        assert_eq!(Solution::largest_submatrix(matrix), 0);
    }
}
