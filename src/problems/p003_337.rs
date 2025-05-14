pub struct Solution;

const MOD: i32 = 1_000_000_007;
const N: usize = 26;

type Vector = [i32; N];
type Matrix = [[i32; N]; N];

#[inline]
fn mod_add(a: i32, b: i32) -> i32 {
    (a + b) % MOD
}

#[inline]
fn mod_mul(a: i32, b: i32) -> i32 {
    (a as i64 * b as i64 % MOD as i64) as i32
}

#[allow(clippy::needless_range_loop)]
fn mat_mul(a: &Matrix, b: &Matrix) -> Matrix {
    let mut c = [[0; N]; N];
    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                c[i][j] = mod_add(c[i][j], mod_mul(a[i][k], b[k][j]));
            }
        }
    }
    c
}

#[allow(clippy::needless_range_loop)]
fn mat_pow(a: &Matrix, n: i32) -> Matrix {
    let mut res = [[0; N]; N];
    for i in 0..N {
        res[i][i] = 1;
    }
    let mut a = *a;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            res = mat_mul(&res, &a);
        }
        a = mat_mul(&a, &a);
        n >>= 1;
    }
    res
}

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
        let cnt: Vector = {
            let mut cnt = [0; N];
            for c in s.chars() {
                cnt[(c as u8 - b'a') as usize] += 1;
            }
            cnt
        };

        let mut mat = [[0; N]; N];

        for i in 0..N {
            for j in 1..=nums[i] as usize {
                mat[(i + j) % 26][i] = 1;
            }
        }

        let mat = mat_pow(&mat, t);

        let mut res = [0; N];
        for i in 0..N {
            for j in 0..N {
                res[i] = mod_add(res[i], mod_mul(mat[i][j], cnt[j]));
            }
        }

        res.iter().fold(0, |acc, &x| mod_add(acc, x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = "abcyy".to_string();
        let t = 2;
        let nums = vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2,
        ];
        assert_eq!(Solution::length_after_transformations(s, t, nums), 7);
    }

    #[test]
    fn sample2() {
        let s = "azbk".to_string();
        let t = 1;
        let nums = vec![
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        ];
        assert_eq!(Solution::length_after_transformations(s, t, nums), 8);
    }
}
