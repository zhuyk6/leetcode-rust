pub struct Solution;

const MOD: i32 = 1_000_000_007;

#[inline]
fn mul(a: i32, b: i32) -> i32 {
    ((a as i64 * b as i64) % MOD as i64) as i32
}

impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        let mut acc = 1;
        for i in 2..=n {
            acc = mul(acc, mul(i, (mul(2, i) - 1 + MOD) % MOD));
        }
        acc
    }
}
