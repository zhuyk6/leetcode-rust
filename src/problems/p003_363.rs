pub struct Solution;

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn max_collected_fruits(mut fruits: Vec<Vec<i32>>) -> i32 {
        let n = fruits.len();
        let mut ans = 0;

        // first: from (0, 0) to (n - 1, n - 1) diagonal
        for i in 0..n {
            ans += fruits[i][i];
            fruits[i][i] = 0;
        }

        let mut dp = vec![vec![0; n]; n];

        // second: from (0, n - 1) to (n - 1, n - 1), never cross diagonal
        for i in 0..n {
            for j in (n - 1 - i)..n {
                dp[i][j] += fruits[i][j];

                if i + 1 < n {
                    dp[i + 1][j - 1] = dp[i + 1][j - 1].max(dp[i][j]);
                    dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
                    if j + 1 < n {
                        dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j]);
                    }
                }
            }
        }
        ans += dp[n - 1][n - 1];

        // third: from (n - 1, 0) to (n - 1, n - 1), never cross diagonal
        for row in &mut dp {
            row.fill(0);
        }
        for j in 0..n {
            for i in (n - 1 - j)..n {
                dp[i][j] += fruits[i][j];

                if j + 1 < n {
                    dp[i - 1][j + 1] = dp[i - 1][j + 1].max(dp[i][j]);
                    dp[i][j + 1] = dp[i][j + 1].max(dp[i][j]);
                    if i + 1 < n {
                        dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j]);
                    }
                }
            }
        }
        ans += dp[n - 1][n - 1];

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let fruits = nested_vec![
            [1, 2, 3, 4],
            [5, 6, 8, 7],
            [9, 10, 11, 12],
            [13, 14, 15, 16]
        ];
        assert_eq!(Solution::max_collected_fruits(fruits), 100);
    }

    #[test]
    fn sample2() {
        let fruits = nested_vec![[1, 1], [1, 1],];
        assert_eq!(Solution::max_collected_fruits(fruits), 4);
    }
}
