pub struct Solution;

impl Solution {
    pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        const MAXV: i32 = 50;
        const MOD: i32 = 1_000_000_007;

        let mut dp: Vec<[i32; MAXV as usize + 1]> = vec![[0; MAXV as usize + 1]; n];

        // dp[0]
        for a in 0..=nums[0] as usize {
            dp[0][a] = 1;
        }

        // dp[i]
        for i in 1..n {
            for a in 0..=nums[i] as usize {
                let b = nums[i] as usize - a;
                for aa in 0..=a.min(nums[i - 1] as usize) {
                    let bb = nums[i - 1] as usize - aa;
                    if bb >= b {
                        dp[i][a] = (dp[i][a] + dp[i - 1][aa]) % MOD;
                    }
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
