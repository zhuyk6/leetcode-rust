pub struct Solution;

const MOD: i32 = 1_000_000_007;
const N: usize = 26;

type Vector = [i32; N];
type Matrix = [[i32; N]; N];

#[inline(always)]
fn mod_add(a: i32, b: i32) -> i32 {
    (a + b) % MOD
}

#[inline(always)]
fn mod_mul(a: i32, b: i32) -> i32 {
    (a as i64 * b as i64 % MOD as i64) as i32
}

fn mat_mul(a: Matrix, b: Matrix) -> Matrix {
    let mut res = [[0; N]; N];
    #[allow(clippy::needless_range_loop)]
    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                res[i][j] = mod_add(res[i][j], mod_mul(a[i][k], b[k][j]));
            }
        }
    }
    res
}

fn mat_pow(a: Matrix, n: u32) -> Matrix {
    let mut res = [[0; N]; N];
    #[allow(clippy::needless_range_loop)]
    for i in 0..N {
        res[i][i] = 1;
    }
    let mut a = a;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            res = mat_mul(res, a);
        }
        a = mat_mul(a, a);
        n >>= 1;
    }
    res
}

impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        let cnt: Vector = s
            .chars()
            .map(|c| c as i32 - 'a' as i32)
            .fold([0; N], |mut acc, x| {
                acc[x as usize] += 1;
                acc
            });

        let mut mat: Matrix = [[0; N]; N];
        mat[0][N - 1] = 1;
        mat[1][N - 1] = 1;
        #[allow(clippy::needless_range_loop)]
        for i in 1..N {
            mat[i][i - 1] = 1;
        }

        let mat = mat_pow(mat, t as u32);

        let mut c0 = [0; N];
        #[allow(clippy::needless_range_loop)]
        for i in 0..N {
            for j in 0..N {
                c0[i] = mod_add(c0[i], mod_mul(mat[i][j], cnt[j]));
            }
        }

        c0.iter().fold(0, |acc, &x| mod_add(acc, x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = "abcyy".to_string();
        let t = 2;
        assert_eq!(Solution::length_after_transformations(s, t), 7);
    }

    #[test]
    fn sample2() {
        let s = "azbk".to_string();
        let t = 1;
        assert_eq!(Solution::length_after_transformations(s, t), 5);
    }
}
