pub struct Solution;

fn factor(mut x: i32) -> Vec<(i32, u32)> {
    let mut factors = vec![];
    let mut p = 2;
    while p * p <= x {
        if x % p == 0 {
            factors.push((p, 0));
            while x % p == 0 {
                factors.last_mut().unwrap().1 += 1;
                x /= p;
            }
        }
        p += 1;
    }
    if x > 1 {
        factors.push((x, 1));
    }
    factors
}

fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 { x } else { gcd(y, x % y) }
}

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i64 {
        let mut factors = factor(k);
        dbg!(&factors);
        factors.dedup();

        const MAXV: i32 = 100_000;
        let mut counter = vec![0i64; MAXV as usize + 1];

        fn dfs(counter: &mut [i64], acc: i32, x: usize, factors: &[(i32, u32)]) {
            if x >= factors.len() {
                counter[acc as usize] += 1;
                return;
            }
            let mut tmp = acc;
            for _ in 0..=factors[x].1 {
                dfs(counter, tmp, x + 1, factors);
                tmp *= factors[x].0;
            }
        }

        let mut ans = 0i64;
        for v in nums {
            let x = k / gcd(v, k);
            ans += counter[x as usize];

            // let v_factors = factor(v);
            let mut v_factors = Vec::with_capacity(factors.len());
            for p in factors.iter().map(|p| p.0) {
                v_factors.push((p, 0));
                let mut v = v;
                while v % p == 0 {
                    v_factors.last_mut().unwrap().1 += 1;
                    v /= p
                }
            }

            dfs(&mut counter, 1, 0, &v_factors);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![1, 2, 3, 4, 5];
        let k = 2;
        assert_eq!(Solution::count_pairs(nums, k), 7);
    }

    #[test]
    fn sample2() {
        let nums = vec![1, 2, 3, 4];
        let k = 5;
        assert_eq!(Solution::count_pairs(nums, k), 0);
    }
}
