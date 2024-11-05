pub struct Solution;

const MOD: i32 = 1_000_000_007;
const BIAS: usize = 1000;

impl Solution {
    pub fn count_winning_sequences(s: String) -> i32 {
        let seq = s.as_bytes();
        let n = seq.len();

        #[inline]
        fn card(c: u8) -> usize {
            match c {
                b'F' => 0,
                b'E' => 1,
                b'W' => 2,
                _ => panic!("error!"),
            }
        }

        #[inline]
        fn score(c0: usize, c1: usize, b: usize) -> usize {
            if c0 == c1 {
                b
            } else if c0 == (c1 + 1) % 3 {
                b + 1
            } else {
                b - 1
            }
        }

        let mut dp = vec![vec![[0; 3]; BIAS * 2 + 1]; n];

        dp[0][BIAS][card(seq[0])] = 1;
        dp[0][BIAS - 1][(card(seq[0]) + 1) % 3] = 1;
        dp[0][BIAS + 1][(card(seq[0]) + 2) % 3] = 1;

        for i in 0..(n - 1) {
            for s in 0..=2 * BIAS {
                for c in 0..3 {
                    if dp[i][s][c] > 0 {
                        for cc in 0..3 {
                            if c == cc {
                                continue;
                            }
                            let ss = score(card(seq[i + 1]), cc, s);
                            dp[i + 1][ss][cc] = (dp[i + 1][ss][cc] + dp[i][s][c]) % MOD;
                        }
                    }
                }
            }
        }

        let mut ans = 0;
        for s in BIAS + 1..=2 * BIAS {
            for c in 0..3 {
                ans = (ans + dp[n - 1][s][c]) % MOD;
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
        let s = "FFF".to_string();
        assert_eq!(Solution::count_winning_sequences(s), 3);
    }

    #[test]
    fn sample2() {
        let s = "FWEFW".to_string();
        assert_eq!(Solution::count_winning_sequences(s), 18);
    }
}
