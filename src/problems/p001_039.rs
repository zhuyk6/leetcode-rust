pub struct Solution;

impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let n = values.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![i32::MAX; n]; n];

        // initilize dp[i][i+1] = 0
        #[allow(clippy::needless_range_loop)]
        for i in 0..n - 1 {
            dp[i][i + 1] = 0;
        }

        for l in 2..=n - 1 {
            for i in 0..n - l {
                let j = i + l;
                for k in i + 1..j {
                    dp[i][j] =
                        dp[i][j].min(dp[i][k] + dp[k][j] + values[i] * values[j] * values[k]);
                }
            }
        }

        dp[0][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let values = vec![1, 2, 3];
        assert_eq!(Solution::min_score_triangulation(values), 6);
    }

    #[test]
    fn sample2() {
        let values = vec![3, 7, 4, 5];
        assert_eq!(Solution::min_score_triangulation(values), 144);
    }

    #[test]
    fn sample3() {
        let values = vec![1, 3, 1, 4, 1, 5];
        assert_eq!(Solution::min_score_triangulation(values), 13);
    }
}
