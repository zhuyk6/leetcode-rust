pub struct Solution;

impl Solution {
    pub fn beautiful_partitions(s: String, k: i32, min_length: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let max_k = k as usize;
        let min_l = min_length as usize;

        const MOD: i32 = 1_000_000_007;
        let mut dp = vec![vec![0; max_k + 1]; n];

        #[inline]
        fn is_prime(x: u8) -> bool {
            matches!(x, b'2' | b'3' | b'5' | b'7')
        }

        if !is_prime(s[0]) {
            return 0;
        }

        for i in min_l - 1..n {
            if !is_prime(s[i]) {
                dp[i][1] = 1;
            }
        }

        let mut acc = vec![0; max_k + 1];

        for i in 0..n {
            if let Some(j) = i.checked_sub(min_l)
                && is_prime(s[j + 1])
            {
                #[allow(clippy::needless_range_loop)]
                for k in 2..=max_k {
                    acc[k] = (acc[k] + dp[j][k - 1]) % MOD;
                }
            }
            if is_prime(s[i]) {
                continue;
            }

            dp[i][2..].copy_from_slice(&acc[2..]);
        }

        dp[n - 1][max_k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = "23542185131".to_string();
        let k = 3;
        let min_length = 2;
        assert_eq!(Solution::beautiful_partitions(s, k, min_length), 3);
    }

    #[test]
    fn sample2() {
        let s = "23542185131".to_string();
        let k = 3;
        let min_length = 3;
        assert_eq!(Solution::beautiful_partitions(s, k, min_length), 1);
    }

    #[test]
    fn sample3() {
        let s = "3312958".to_string();
        let k = 3;
        let min_length = 1;
        assert_eq!(Solution::beautiful_partitions(s, k, min_length), 1);
    }
}
