pub struct Solution;

impl Solution {
    pub fn number_of_permutations_old(n: i32, mut requirements: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let m = requirements.iter().map(|r| r[1]).max().unwrap() as usize;

        requirements.sort_unstable_by_key(|r| r[0]);

        let mut dp = vec![vec![0; m + 1]; n];

        let mut r = 0;

        const MOD: i32 = 1_000_000_007;

        dp[0][0] = 1;
        if requirements[0][0] == 0 && requirements[0][1] != 0 {
            dp[0][0] = 0;
        }

        for i in 1..n {
            while r < requirements.len() && requirements[r][0] < i as i32 {
                r += 1;
            }
            for j in 0..=m.min(i * (i + 1) / 2) {
                if requirements[r][0] == i as i32 && requirements[r][1] != j as i32 {
                    dp[i][j] = 0;
                } else {
                    for k in j.saturating_sub(i)..=j {
                        dp[i][j] = (dp[i][j] + dp[i - 1][k]) % MOD;
                    }
                }
            }
        }

        // dbg!(&dp);

        dp[n - 1][requirements.last().unwrap()[1] as usize]
    }

    pub fn number_of_permutations(n: i32, mut requirements: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let m = requirements.iter().map(|r| r[1]).max().unwrap() as usize;

        requirements.sort_unstable_by_key(|r| r[0]);

        let mut dp = vec![vec![0; m + 1]; n];
        let mut sum = vec![0; m + 1];

        let mut r = 0;

        const MOD: i32 = 1_000_000_007;

        if requirements[0][0] == 0 && requirements[0][1] != 0 {
            return 0;
        } else {
            dp[0][0] = 1;
            sum.fill(1);
        }

        #[allow(clippy::needless_range_loop)]
        for i in 1..n {
            while r < requirements.len() && requirements[r][0] < i as i32 {
                r += 1;
            }
            for j in 0..=m.min(i * (i + 1) / 2) {
                if requirements[r][0] == i as i32 && requirements[r][1] != j as i32 {
                    dp[i][j] = 0;
                } else {
                    let p = j.saturating_sub(i);
                    let tmp = if p > 0 { sum[p - 1] } else { 0 };
                    dp[i][j] = (sum[j] + (MOD - tmp)) % MOD;
                }
            }

            // update sum
            sum[0] = dp[i][0];
            for j in 1..=m {
                sum[j] = (sum[j - 1] + dp[i][j]) % MOD;
            }
        }

        // dbg!(&dp);

        dp[n - 1][requirements.last().unwrap()[1] as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let n = 3;
        let requirements = nested_vec![[2, 2], [0, 0]];
        assert_eq!(Solution::number_of_permutations(n, requirements), 2);
    }

    #[test]
    fn sample2() {
        let n = 3;
        let requirements = nested_vec![[2, 2], [1, 1], [0, 0]];
        assert_eq!(Solution::number_of_permutations(n, requirements), 1);
    }

    #[test]
    fn sample3() {
        let n = 2;
        let requirements = nested_vec![[0, 0], [1, 0]];
        assert_eq!(Solution::number_of_permutations(n, requirements), 1);
    }

    #[test]
    fn wrong1() {
        let n = 3;
        let requirements = nested_vec![[2, 2], [0, 1]];
        assert_eq!(Solution::number_of_permutations(n, requirements), 0);
    }

    #[test]
    fn wrong2() {
        let n = 3;
        let requirements = nested_vec![[2, 3], [0, 0]];
        assert_eq!(Solution::number_of_permutations(n, requirements), 1);
    }
}
