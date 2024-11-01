pub struct Solution;

impl Solution {
    pub fn max_energy_boost(a: Vec<i32>, b: Vec<i32>) -> i64 {
        let n = a.len();
        let mut dp = vec![[0i64; 2]; n];

        dp[0][0] = a[0] as i64;
        dp[0][1] = b[0] as i64;

        for i in 1..n {
            dp[i][0] = i64::max(dp[i - 1][0] + a[i] as i64, dp[i - 1][1]);
            dp[i][1] = i64::max(dp[i - 1][1] + b[i] as i64, dp[i - 1][0]);
        }

        dp[n - 1].iter().copied().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let a = vec![1, 3, 1];
        let b = vec![3, 1, 1];
        assert_eq!(Solution::max_energy_boost(a, b), 5);
    }

    #[test]
    fn sample2() {
        let a = vec![4, 1, 1];
        let b = vec![1, 1, 3];
        assert_eq!(Solution::max_energy_boost(a, b), 7);
    }
}
