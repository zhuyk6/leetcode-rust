pub struct Solution;

impl Solution {
    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let num = k as usize;
        assert!(num >= 1);

        fn cost(l: usize, r: usize, s: &[u8]) -> i32 {
            assert!(l <= r);
            let len = r - l + 1;
            s[l..=r]
                .iter()
                .zip(s[l..=r].iter().rev())
                .take(len / 2)
                .map(|(a, b)| if a == b { 0 } else { 1 })
                .sum()
        }

        let mut dp = vec![vec![i32::MAX; num + 1]; n];
        dp[0][1] = 0;
        for i in 1..n {
            dp[i][1] = cost(0, i, s);
            for j in 0..i {
                for k in 2..=num {
                    if dp[j][k - 1] == i32::MAX {
                        break;
                    }
                    dp[i][k] = dp[i][k].min(dp[j][k - 1] + cost(j + 1, i, s));
                }
            }
        }

        dp[n - 1][num]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = "abc".to_string();
        let k = 2;
        assert_eq!(Solution::palindrome_partition(s, k), 1);
    }

    #[test]
    fn sample2() {
        let s = "aabbc".to_string();
        let k = 3;
        assert_eq!(Solution::palindrome_partition(s, k), 0);
    }
}
