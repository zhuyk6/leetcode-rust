pub struct Solution;

const MOD: i32 = 1_000_000_007;

fn mul_mod(a: i32, b: i32) -> i32 {
    (a as i64 * b as i64 % MOD as i64) as i32
}

fn pow_mod(base: i32, exp: u32) -> i32 {
    let mut result = 1;
    let mut b = base;
    let mut e = exp;

    while e > 0 {
        if e % 2 == 1 {
            result = mul_mod(result, b);
        }
        b = mul_mod(b, b);
        e /= 2;
    }
    result
}

fn inv_mod(a: i32) -> i32 {
    pow_mod(a, MOD as u32 - 2)
}

fn choice(n: i32, k: i32) -> i32 {
    if k > n {
        return 0;
    }
    let mut numerator = 1;
    let mut denominator = 1;

    for i in 0..k {
        numerator = mul_mod(numerator, n - i);
        denominator = mul_mod(denominator, i + 1);
    }

    mul_mod(numerator, inv_mod(denominator))
}

impl Solution {
    pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        mul_mod(
            m,
            mul_mod(pow_mod(m - 1, (n - k - 1) as u32), choice(n - 1, k)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let n = 3;
        let m = 2;
        let k = 1;
        assert_eq!(Solution::count_good_arrays(n, m, k), 4);
    }

    #[test]
    fn sample2() {
        let n = 4;
        let m = 2;
        let k = 2;
        assert_eq!(Solution::count_good_arrays(n, m, k), 6);
    }

    #[test]
    fn sample3() {
        let n = 5;
        let m = 2;
        let k = 0;
        assert_eq!(Solution::count_good_arrays(n, m, k), 2);
    }
}
