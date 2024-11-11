pub struct Solution;

impl Solution {
    pub fn min_cost(n: i32, mut cuts: Vec<i32>) -> i32 {
        cuts.insert(0, 0);
        cuts.push(n);
        cuts.sort_unstable();

        let m = cuts.len();
        let mut dp = vec![vec![i32::MAX; m]; m];

        #[allow(clippy::needless_range_loop)]
        for i in 0..m - 1 {
            dp[i][i + 1] = 0;
        }
        for l in 2..m {
            for i in 0..m - l {
                let j = i + l;
                for k in i + 1..j {
                    dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j] + cuts[j] - cuts[i]);
                }
            }
        }

        dp[0][m - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let n = 7;
        let cuts = vec![1, 3, 4, 5];
        assert_eq!(Solution::min_cost(n, cuts), 16);
    }

    #[test]
    fn sample2() {
        let n = 9;
        let cuts = vec![5, 6, 1, 4, 2];
        assert_eq!(Solution::min_cost(n, cuts), 22);
    }
}
