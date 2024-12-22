pub struct Solution;

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn sum_of_power(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        assert!(nums.iter().all(|v| *v > 0));

        let k = k as usize;

        let mut dp = vec![vec![0; k + 1]; n];

        for (&v, i) in nums.iter().zip(0..) {
            dp[i][0] = 1;
            let v = v as usize;
            if v <= k {
                dp[i][v] = (dp[i][v] + 1) % MOD;
            }
            for j in 0..i {
                for u in 0..=k {
                    dp[i][u] = (dp[i][u] + dp[j][u]) % MOD;
                }
                if k >= v {
                    for u in 0..=k - v {
                        dp[i][u + v] = (dp[i][u + v] + dp[j][u]) % MOD;
                    }
                }
            }
        }

        (0..n).fold(0, |acc, i| (dp[i][k] + acc) % MOD)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![1, 2, 3];
        let k = 3;
        assert_eq!(Solution::sum_of_power(nums, k), 6)
    }

    #[test]
    fn sample2() {
        let nums = vec![2, 3, 3];
        let k = 5;
        assert_eq!(Solution::sum_of_power(nums, k), 4)
    }

    #[test]
    fn sample3() {
        let nums = vec![1, 2, 3];
        let k = 7;
        assert_eq!(Solution::sum_of_power(nums, k), 0)
    }
}
