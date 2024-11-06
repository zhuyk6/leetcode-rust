pub struct Solution;

impl Solution {
    pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
        let n = nums.len(); // n <= 2_000
        const MAXV: i32 = 1_000;
        const MOD: i32 = 1_000_000_007;

        let mut dp: Vec<[i32; MAXV as usize + 1]> = vec![[0; MAXV as usize + 1]; n];

        // dp[0]
        for a in 0..=nums[0] as usize {
            dp[0][a] = 1;
        }

        // dp[i]
        for i in 1..n {
            let di = nums[i] - nums[i - 1];
            if di > 0 {
                let di = di as usize;
                // di <= a <= nums[i]
                // 0 <= aa <= min(a - di, nums[i-1])
                dp[i][di] = dp[i - 1][0];
                for a in di + 1..=nums[i] as usize {
                    if a - di <= nums[i - 1] as usize {
                        dp[i][a] = (dp[i][a - 1] + dp[i - 1][a - di]) % MOD;
                    } else {
                        dp[i][a] = dp[i][a - 1];
                    }
                }
            } else {
                // 0 <= a <= nums[i] <= nums[i-1]
                // 0 <= aa <= min(a, nums[i-1]) = a
                dp[i][0] = dp[i - 1][0];
                for a in 1..=nums[i] as usize {
                    dp[i][a] = (dp[i][a - 1] + dp[i - 1][a]) % MOD;
                }
            }
        }

        let mut ans = 0;
        for a in 0..=nums[n - 1] as usize {
            ans = (ans + dp[n - 1][a]) % MOD;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![2, 3, 2];
        assert_eq!(Solution::count_of_pairs(nums), 4);
    }

    #[test]
    fn sample2() {
        let nums = vec![5, 5, 5, 5];
        assert_eq!(Solution::count_of_pairs(nums), 126);
    }
}
