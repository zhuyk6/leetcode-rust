pub struct Solution;

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn count_palindromes(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();

        let mut suffix_sum1 = vec![[0; 10]; n];
        let mut suffix_sum2 = vec![[0; 100]; n];

        suffix_sum1[n - 1][(s[n - 1] - b'0') as usize] = 1;
        for i in (0..(n - 1)).rev() {
            let c = (s[i] - b'0') as usize;

            suffix_sum2[i] = suffix_sum2[i + 1];
            for cc in 0..10 {
                let v = c * 10 + cc;
                suffix_sum2[i][v] += suffix_sum1[i + 1][cc];
            }

            suffix_sum1[i] = suffix_sum1[i + 1];
            suffix_sum1[i][c] += 1;
        }

        let mut ans = 0;
        let mut prefix_sum1 = vec![[0; 10]; n];
        let mut prefix_sum2 = vec![[0; 100]; n];

        prefix_sum1[0][(s[0] - b'0') as usize] = 1;
        for i in 1..(n - 1) {
            let c = (s[i] - b'0') as usize;

            for u in 0..100 {
                let (x, y) = (u / 10, u % 10);
                let v = y * 10 + x;
                let tmp =
                    (prefix_sum2[i - 1][u] as i64 * suffix_sum2[i + 1][v] as i64) % MOD as i64;
                ans = (ans + tmp as i32) % MOD;
            }

            prefix_sum2[i] = prefix_sum2[i - 1];
            for cc in 0..10 {
                let v = cc * 10 + c;
                prefix_sum2[i][v] += prefix_sum1[i - 1][cc];
            }

            prefix_sum1[i] = prefix_sum1[i - 1];
            prefix_sum1[i][c] += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = "103301".to_string();
        assert_eq!(Solution::count_palindromes(s), 2);
    }

    #[test]
    fn sample2() {
        let s = "0000000".to_string();
        assert_eq!(Solution::count_palindromes(s), 21);
    }

    #[test]
    fn sample3() {
        let s = "9999900000".to_string();
        assert_eq!(Solution::count_palindromes(s), 2);
    }
}
