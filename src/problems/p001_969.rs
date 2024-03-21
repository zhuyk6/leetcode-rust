struct Solution;

#[inline]
fn mul(x: i64, y: i64, p: i64) -> i64 {
    x * y % p
}

fn pow(mut a: i64, n: i64, p: i64) -> i64 {
    let mut acc = 1;
    let mut bit = 1;
    while bit <= n {
        if n & bit > 0 {
            acc = mul(acc, a, p);
        }
        bit <<= 1;
        a = mul(a, a, p);
    }
    acc
}

#[allow(unused)]
impl Solution {
    pub fn min_non_zero_product(p: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let p = p as i64;
        let x = (pow(2, p, MOD) - 1 + MOD) % MOD;
        let a = (pow(2, p, MOD) - 2 + MOD) % MOD;
        let n = (pow(2, p - 1, MOD - 1) - 1 + (MOD - 1)) % (MOD - 1);
        let y = pow(a, n, MOD);
        (x * y % MOD) as i32
    }
}
