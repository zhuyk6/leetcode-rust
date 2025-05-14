pub struct Solution;

const MOD: i32 = 1_000_000_007;

fn mod_add(a: i32, b: i32) -> i32 {
    (a + b) % MOD
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

impl Solution {
    pub fn subsequence_pair_count(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = *nums.iter().max().unwrap() as usize + 1;

        let mut dp = vec![vec![vec![0; m]; m]; n];

        dp[0][0][0] = 1;
        dp[0][nums[0] as usize][0] = 1;
        dp[0][0][nums[0] as usize] = 1;

        for (&v, i) in nums[1..].iter().zip(1..) {
            // not use v
            for p in 0..m {
                for q in 0..m {
                    dp[i][p][q] = dp[i - 1][p][q];
                }
            }

            // use v
            for p in 0..m {
                for q in 0..m {
                    let pp = gcd(p, v as usize);
                    let qq = gcd(q, v as usize);
                    dp[i][pp][q] = mod_add(dp[i][pp][q], dp[i - 1][p][q]);
                    dp[i][p][qq] = mod_add(dp[i][p][qq], dp[i - 1][p][q]);
                }
            }
        }

        let mut ans = 0;
        for p in 1..m {
            ans = mod_add(ans, dp[n - 1][p][p])
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
        assert_eq!(Solution::subsequence_pair_count(nums), 10);
    }

    #[test]
    fn sample2() {
        let nums = vec![10, 20, 30];
        assert_eq!(Solution::subsequence_pair_count(nums), 2);
    }
}
