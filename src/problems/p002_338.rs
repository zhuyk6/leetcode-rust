pub struct Solution;

const MOD: i32 = 1_000_000_007;

#[derive(Debug, Clone, Copy, Default)]
struct ModV(i32);

impl ModV {
    #[inline]
    fn new(x: i32) -> Self {
        Self(x % MOD)
    }

    fn pow(self, n: u32) -> Self {
        let mut res = ModV::new(1);
        let mut a = self;
        let mut n = n;
        while n > 0 {
            if n & 1 == 1 {
                res = res * a;
            }
            a = a * a;
            n >>= 1;
        }
        res
    }

    fn inv(self) -> Self {
        self.pow(MOD as u32 - 2)
    }
}

impl std::ops::Add for ModV {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        ModV((self.0 + rhs.0) % MOD)
    }
}

impl std::ops::Mul for ModV {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        ModV((self.0 as i64 * rhs.0 as i64 % MOD as i64) as i32)
    }
}

impl std::ops::Div for ModV {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

#[allow(unused)]
#[inline]
fn add(a: i32, b: i32) -> i32 {
    (a + b) % MOD
}

#[inline]
fn mul(a: i32, b: i32) -> i32 {
    ((a as i64 * b as i64) % MOD as i64) as i32
}

fn pow(a: i32, n: i32) -> i32 {
    let mut res = 1;
    let mut a = a;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            res = mul(res, a);
        }
        a = mul(a, a);
        n >>= 1;
    }
    res
}

#[inline]
fn mod_inverse(a: i32) -> i32 {
    pow(a, MOD - 2)
}

#[allow(unused)]
fn choose(n: i32, k: i32) -> i32 {
    if k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }
    let mut res = 1;
    for i in 1..=k {
        res = mul(res, n - i + 1);
        res = mul(res, mod_inverse(i));
    }
    res
}

fn choose_mod_v(n: i32, k: i32) -> ModV {
    if k > n {
        return ModV::new(0);
    }
    if k == 0 || k == n {
        return ModV::new(1);
    }
    let mut res = ModV::new(1);
    for i in 1..=k {
        res = res * ModV::new(n - i + 1);
        res = res * ModV::new(i).inv();
    }
    res
}

struct Solver {
    n: i32,
    max_value: i32,
    #[allow(unused)]
    primes: Vec<i32>,
    is_prime: Vec<bool>,
    filter_prime: Vec<i32>,
}

impl Solver {
    fn new(n: i32, max_value: i32) -> Self {
        let mut is_prime = vec![true; (max_value + 1) as usize];
        let mut primes = vec![];
        let mut filter_prime = vec![0; (max_value + 1) as usize];

        is_prime[0] = false;
        is_prime[1] = false;

        for i in 2..=max_value {
            if is_prime[i as usize] {
                primes.push(i);
            }

            for &p in &primes {
                if i * p > max_value {
                    break;
                }
                is_prime[(i * p) as usize] = false;
                filter_prime[(i * p) as usize] = p;

                if i % p == 0 {
                    break;
                }
            }
        }

        Self {
            n,
            max_value,
            primes,
            is_prime,
            filter_prime,
        }
    }

    #[allow(unused)]
    fn factor(x: i32) -> Vec<(i32, u32)> {
        let mut factors = vec![];

        let mut x = x;
        let mut d = 2;
        while d * d <= x {
            let mut count = 0;
            while x % d == 0 {
                x /= d;
                count += 1;
            }
            if count > 0 {
                factors.push((d, count));
            }
            d += 1;
        }

        if x > 1 {
            factors.push((x, 1));
        }

        factors
    }

    fn get_prime_factors(&self, x: i32) -> Vec<(i32, u32)> {
        let mut factors = vec![];

        if x <= 1 {
            return factors;
        }

        let mut x = x;
        while x > 1 {
            if self.is_prime[x as usize] {
                factors.push((x, 1));
                break;
            }
            let p = self.filter_prime[x as usize];
            let mut count = 0;
            while x % p == 0 {
                x /= p;
                count += 1;
            }
            factors.push((p, count));
        }

        factors
    }

    /// Calculates the answer with the ending value of `x`.
    fn _solve(&self, x: i32) -> i32 {
        let factors = self.get_prime_factors(x);

        let mut ans = 1;
        for (_, c) in factors {
            ans = mul(ans, choose(self.n - 1 + c as i32, c as i32));
        }

        ans
    }

    fn _solve_mod_v(&self, x: i32) -> ModV {
        let factors = self.get_prime_factors(x);

        let mut ans = ModV::new(1);
        for (_, c) in factors {
            ans = ans * choose_mod_v(self.n - 1 + c as i32, c as i32);
        }

        ans
    }

    fn solve(&self) -> i32 {
        // (1..=self.max_value).map(|i| self._solve(i)).fold(0, add)
        (1..=self.max_value)
            .map(|i| self._solve_mod_v(i))
            .fold(ModV::new(0), std::ops::Add::add)
            .0
    }
}

impl Solution {
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        let solver = Solver::new(n, max_value);
        solver.solve()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let n = 2;
        let max_value = 5;
        assert_eq!(Solution::ideal_arrays(n, max_value), 10);
    }

    #[test]
    fn sample2() {
        let n = 5;
        let max_value = 3;
        assert_eq!(Solution::ideal_arrays(n, max_value), 11);
    }

    #[test]
    fn sample3() {
        let n = 5;
        let max_value = 10;
        assert_eq!(Solution::ideal_arrays(n, max_value), 136);
    }
}
