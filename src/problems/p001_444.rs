pub struct Solution;

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let m = pizza.len();
        let n = pizza[0].len();
        let pre_sum = {
            let mut s = vec![vec![0u32; n]; m];

            s[0][0] = if b'A' == pizza[0].as_bytes()[0] { 1 } else { 0 };

            for j in 1..n {
                s[0][j] = s[0][j - 1] + if b'A' == pizza[0].as_bytes()[j] { 1 } else { 0 };
            }
            for i in 1..m {
                s[i][0] = s[i - 1][0] + if b'A' == pizza[i].as_bytes()[0] { 1 } else { 0 };
            }
            for i in 1..m {
                for j in 1..n {
                    s[i][j] = if b'A' == pizza[i].as_bytes()[j] { 1 } else { 0 }
                        + s[i - 1][j]
                        + s[i][j - 1]
                        - s[i - 1][j - 1];
                }
            }

            s
        };

        let sum = move |r0: usize, r1: usize, c0: usize, c1: usize| -> u32 {
            if r0 > 0 && c0 > 0 {
                pre_sum[r1][c1] + pre_sum[r0 - 1][c0 - 1]
                    - pre_sum[r1][c0 - 1]
                    - pre_sum[r0 - 1][c1]
            } else if r0 > 0 {
                pre_sum[r1][c1] - pre_sum[r0 - 1][c1]
            } else if c0 > 0 {
                pre_sum[r1][c1] - pre_sum[r1][c0 - 1]
            } else {
                pre_sum[r1][c1]
            }
        };

        let max_k = k as usize;
        let mut dp = vec![vec![vec![0; n]; m]; max_k];

        if sum(0, m - 1, 0, n - 1) == 0 {
            return 0;
        } else {
            dp[0][0][0] = 1;
        }

        for k in 0..max_k - 1 {
            for r in 0..m {
                for c in 0..n {
                    if dp[k][r][c] == 0 {
                        continue;
                    }
                    for i in r + 1..m {
                        if sum(r, i - 1, c, n - 1) > 0 {
                            dp[k + 1][i][c] = (dp[k + 1][i][c] + dp[k][r][c]) % MOD;
                        }
                    }
                    for j in c + 1..n {
                        if sum(r, m - 1, c, j - 1) > 0 {
                            dp[k + 1][r][j] = (dp[k + 1][r][j] + dp[k][r][c]) % MOD;
                        }
                    }
                }
            }
        }

        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if sum(i, m - 1, j, n - 1) > 0 {
                    ans = (ans + dp[max_k - 1][i][j]) % MOD;
                }
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
        let pizza = ["A..", "AAA", "..."];
        let pizza = pizza.into_iter().map(String::from).collect();
        let k = 3;
        assert_eq!(Solution::ways(pizza, k), 3);
    }

    #[test]
    fn sample2() {
        let pizza = ["A..", "AA.", "..."];
        let pizza = pizza.into_iter().map(String::from).collect();
        let k = 3;
        assert_eq!(Solution::ways(pizza, k), 1);
    }

    #[test]
    fn sample3() {
        let pizza = ["A..", "A..", "..."];
        let pizza = pizza.into_iter().map(String::from).collect();
        let k = 1;
        assert_eq!(Solution::ways(pizza, k), 1);
    }
}
