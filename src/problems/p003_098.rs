pub struct Solution;

const MOD: i32 = 1_000_000_007;

#[inline]
fn mul(a: i32, b: i32) -> i32 {
    ((a as i64 * b as i64) % MOD as i64) as i32
}

impl Solution {
    pub fn sum_of_powers(mut nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let max_k = k as usize;
        nums.sort_unstable();

        use std::collections::HashMap;
        let (energy_rank, energies): (HashMap<i32, usize>, Vec<i32>) = {
            let mut energies = vec![];
            for i in 0..n {
                for j in i + 1..n {
                    let e = nums[j] - nums[i];
                    energies.push(e);
                }
            }
            energies.sort_unstable();
            energies.dedup();
            (
                energies
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(i, v)| (v, i))
                    .collect(),
                energies,
            )
        };
        dbg!(&energy_rank);

        let m = energy_rank.len();
        let mut dp = vec![vec![vec![0; m]; max_k + 1]; n];

        for i in 0..n {
            for j in i + 1..n {
                let e = nums[j] - nums[i];
                let rank = energy_rank[&e];
                dp[j][2][rank] = (dp[j][2][rank] + 1) % MOD;
            }
        }

        for k in 3..=max_k {
            for i in 0..n {
                for j in 0..i {
                    let e = nums[i] - nums[j];
                    let rank = energy_rank[&e];
                    for l in 0..rank {
                        dp[i][k][l] = (dp[i][k][l] + dp[j][k - 1][l]) % MOD;
                    }
                    for l in rank..m {
                        dp[i][k][rank] = (dp[i][k][rank] + dp[j][k - 1][l]) % MOD;
                    }
                }
            }
        }

        let mut ans = 0;
        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            for rank in 0..m {
                ans = (ans + mul(dp[i][max_k][rank], energies[rank])) % MOD;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![1, 2, 3, 4];
        let k = 3;
        assert_eq!(Solution::sum_of_powers(nums, k), 4)
    }

    #[test]
    fn sample2() {
        let nums = vec![2, 2];
        let k = 2;
        assert_eq!(Solution::sum_of_powers(nums, k), 0)
    }

    #[test]
    fn sample3() {
        let nums = vec![4, 3, -1];
        let k = 2;
        assert_eq!(Solution::sum_of_powers(nums, k), 10)
    }
}
