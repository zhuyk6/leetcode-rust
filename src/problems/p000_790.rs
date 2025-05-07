pub struct Solution;

const MOD: i32 = 1_000_000_007;

#[inline(always)]
fn add(a: i32, b: i32) -> i32 {
    (a + b) % MOD
}

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let n = n as usize;

        let mut dp = [[0; 3]; 3];
        dp[0][0] = 1;

        for i in 0..n {
            let (i, j, k) = (i % 3, (i + 1) % 3, (i + 2) % 3);

            // dp[i][0]
            dp[j][0] = add(dp[j][0], dp[i][0]);
            dp[j][1] = add(dp[j][1], dp[i][0]);
            dp[j][2] = add(dp[j][2], dp[i][0]);
            dp[k][0] = add(dp[k][0], dp[i][0]);

            // dp[i][1]
            dp[j][2] = add(dp[j][2], dp[i][1]);
            dp[k][0] = add(dp[k][0], dp[i][1]);

            // dp[i][2]
            dp[j][1] = add(dp[j][1], dp[i][2]);
            dp[k][0] = add(dp[k][0], dp[i][2]);

            dp[i].fill(0);
        }

        dp[n % 3][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let n = 3;
        assert_eq!(Solution::num_tilings(n), 5);
    }

    #[test]
    fn sample2() {
        let n = 1;
        assert_eq!(Solution::num_tilings(n), 1);
    }

    #[test]
    fn sample3() {
        let n = 999;
        assert_eq!(Solution::num_tilings(n), 326038248);
    }
}
