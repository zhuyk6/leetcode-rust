pub struct Solution;

const MOD: i32 = 1_000_000_007;

#[inline]
fn mul(x: i32, y: i32) -> i32 {
    ((x as i64 * y as i64) % MOD as i64) as i32
}

fn pow(mut x: i32, n: i32) -> i32 {
    let mut acc = 1;
    let mut b = 1;
    while b <= n {
        if n & b > 0 {
            acc = mul(acc, x);
        }
        b <<= 1;
        x = mul(x, x);
    }
    acc
}

impl Solution {
    pub fn number_of_ways(n: i32, x: i32, y: i32) -> i32 {
        // ans = sum_{k = 1}^{min(x, n)} A(x, k) \times S(n, k) \times y^k
        // A(x, k) = x! / (x - k)!
        // S(n, k) = S(n-1, k-1) + k \times S(n-1, k)
        let mut a = vec![0; x as usize + 1];
        a[0] = 1;
        for k in 1..=x as usize {
            a[k] = mul(a[k - 1], x - k as i32 + 1);
        }

        let mut s = vec![vec![0; n as usize + 1]; n as usize + 1];
        s[0][0] = 1;
        for i in 1..=n as usize {
            for j in 1..=i {
                s[i][j] = (s[i - 1][j - 1] + mul(j as i32, s[i - 1][j])) % MOD;
            }
        }

        let mut ans = 0;
        #[allow(clippy::needless_range_loop)]
        for k in 1..=x.min(n) as usize {
            ans = (ans + mul(a[k], mul(s[n as usize][k], pow(y, k as i32)))) % MOD;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let n = 1;
        let x = 2;
        let y = 3;
        assert_eq!(Solution::number_of_ways(n, x, y), 6)
    }

    #[test]
    fn sample2() {
        let n = 5;
        let x = 2;
        let y = 1;
        assert_eq!(Solution::number_of_ways(n, x, y), 32)
    }

    #[test]
    fn sample3() {
        let n = 3;
        let x = 3;
        let y = 4;
        assert_eq!(Solution::number_of_ways(n, x, y), 684)
    }
}
