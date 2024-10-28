pub struct Solution;

const MOD: i32 = 1_000_000_007;

trait AddMod {
    fn add(self, other: Self) -> Self;
}

impl AddMod for i32 {
    #[inline]
    fn add(self, other: Self) -> Self {
        (self + other) % MOD
    }
}

impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let n0 = zero as usize;
        let n1 = one as usize;
        let limit = limit as usize;

        let mut dp = vec![vec![vec![0; 2]; n1 + 1]; n0 + 1];
        // let mut sum0 = vec![vec![vec![0; 2]; n1 + 1]; n0 + 1];
        // let mut sum1 = vec![vec![vec![0; 2]; n1 + 1]; n0 + 1];

        #[allow(clippy::needless_range_loop)]
        for i in 1..=n0.min(limit) {
            dp[i][0][0] = 1;
            // sum0[i][0][0] = i as i32;
            // sum1[i][0][0] = 1;
        }
        #[allow(clippy::needless_range_loop)]
        for i in 1..=n1.min(limit) {
            dp[0][i][1] = 1;
            // sum1[0][i][1] = i as i32;
            // sum0[0][i][1] = 1;
        }

        for i in 1..=n0 {
            for j in 1..=n1 {
                // // place k 0's in the end
                // for k in 1..=i.min(limit) {
                //     dp[i][j][0] = dp[i][j][0].add(dp[i - k][j][1]);
                // }
                // // place k 1's in the end
                // for k in 1..=j.min(limit) {
                //     dp[i][j][1] = dp[i][j][1].add(dp[i][j - k][0]);
                // }

                //     dp[i][j][0] = if i <= limit {
                //         sum0[i - 1][j][1]
                //     } else {
                //         sum0[i - 1][j][1].add(MOD - sum0[i - 1 - limit][j][1])
                //     };

                //     dp[i][j][1] = if j <= limit {
                //         sum1[i][j - 1][0]
                //     } else {
                //         sum1[i][j - 1][0].add(MOD - sum1[i][j - 1 - limit][0])
                //     };

                //     sum0[i][j][0] = sum0[i - 1][j][0].add(dp[i][j][0]);
                //     sum0[i][j][1] = sum0[i - 1][j][1].add(dp[i][j][1]);

                //     sum1[i][j][0] = sum1[i][j - 1][0].add(dp[i][j][0]);
                //     sum1[i][j][1] = sum1[i][j - 1][1].add(dp[i][j][1]);

                //     println!("dp[{i}][{j}] = {:?}", dp[i][j]);
                //     println!("sum0[{i}][{j}] = {:?}", sum0[i][j]);
                //     println!("sum1[{i}][{j}] = {:?}", sum1[i][j]);

                dp[i][j][0] = dp[i - 1][j][0].add(dp[i - 1][j][1]);
                if i > limit {
                    dp[i][j][0] = dp[i][j][0].add(MOD - dp[i - 1 - limit][j][1]);
                }

                dp[i][j][1] = dp[i][j - 1][0].add(dp[i][j - 1][1]);
                if j > limit {
                    dp[i][j][1] = dp[i][j][1].add(MOD - dp[i][j - 1 - limit][0]);
                }
            }
        }

        dp[n0][n1][0].add(dp[n0][n1][1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let zero = 1;
        let one = 1;
        let limit = 2;
        assert_eq!(Solution::number_of_stable_arrays(zero, one, limit), 2);
    }

    #[test]
    fn sample2() {
        let zero = 1;
        let one = 2;
        let limit = 1;
        assert_eq!(Solution::number_of_stable_arrays(zero, one, limit), 1);
    }

    #[test]
    fn sample3() {
        let zero = 3;
        let one = 3;
        let limit = 2;
        assert_eq!(Solution::number_of_stable_arrays(zero, one, limit), 14);
    }
}
