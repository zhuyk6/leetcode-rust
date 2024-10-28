pub struct Solution;

const MOD: i64 = 1_000_000_007;

// fn put(p: usize, q: usize) -> i64 {
//     let p = p as i64;
//     let q = q as i64;
//     let mut ret = 1;
//     for t in (q + 1)..=(q + p) {
//         ret = ret * t % MOD;
//     }
//     ret
// }

#[inline]
fn pow(mut a: i64, n: usize) -> i64 {
    let mut acc = 1;
    let mut bit = 1;
    while bit <= n {
        if n & bit > 0 {
            acc = (acc * a) % MOD;
        }
        a = a * a % MOD;
        bit <<= 1;
    }
    acc
}

#[inline]
fn inv(a: i64) -> i64 {
    pow(a, MOD as usize - 2)
}

#[allow(unused, clippy::needless_range_loop)]
impl Solution {
    pub fn rearrange_sticks(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let max_k = k as usize;

        let mut p = vec![0; n];
        // p(j) := [(n-1-j)!]^(-1)
        {
            p[n - 1] = 1;
            for j in (0..(n - 1)).rev() {
                p[j] = p[j + 1] * (n - 1 - j) as i64 % MOD;
            }
            for v in &mut p {
                *v = inv(*v);
            }
        }

        let mut q = vec![0; n - 1];
        // q(i) := (n-2-i)!
        {
            q[n - 2] = 1;
            for i in (0..(n - 2)).rev() {
                q[i] = q[i + 1] * (n - 2 - i) as i64 % MOD;
            }
        }

        let mut f = vec![vec![0i64; n + 1]; n];
        let mut g = vec![vec![0i64; n + 1]; n];

        f[n - 1][1] = 1;
        for i in (0..(n - 1)).rev() {
            for k in 1..=(n - i) {
                g[i][k - 1] = (g[i + 1][k - 1] + f[i + 1][k - 1] * p[i + 1] % MOD) % MOD;
                f[i][k] = q[i] * g[i][k - 1] % MOD;
                // for j in (i + 1)..n {
                //     f[i][k] = (f[i][k] + f[j][k - 1] * p[j] % MOD) % MOD;
                // }
                // f[i][k] = f[i][k] * q[i] % MOD;
            }
        }

        let mut ans = 0;
        let mut acc = 1;
        for i in (0..n) {
            // ans = (ans + f[i][max_k] * put(i, n - 1 - i) % MOD) % MOD;
            ans = (ans + f[i][max_k] * acc % MOD) % MOD;
            acc = acc * (n - 1 - i) as i64 % MOD;
        }
        ans as i32
    }
}

#[test]
fn test1() {
    let n = 3;
    let k = 2;
    assert_eq!(Solution::rearrange_sticks(n, k), 3);
}

#[test]
fn test2() {
    let n = 5;
    let k = 5;
    assert_eq!(Solution::rearrange_sticks(n, k), 1);
}

#[test]
fn test3() {
    let n = 20;
    let k = 11;
    assert_eq!(Solution::rearrange_sticks(n, k), 647427950);
}

#[test]
fn test4() {
    let n = 5;
    let k = 3;
    assert_eq!(Solution::rearrange_sticks(n, k), 35);
}
