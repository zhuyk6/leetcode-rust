pub struct Solution;

const P: u32 = 1_000_000_000 + 7;

#[inline]
fn mul(x: u32, y: u32) -> u32 {
    (x as u64 * y as u64 % P as u64) as u32
}

fn pow(a: u32, n: u32) -> u32 {
    let mut ans = 1;
    let mut bit = 1;
    let mut acc = a;
    while bit <= n {
        if n & bit > 0 {
            ans = mul(ans, acc);
        }
        acc = mul(acc, acc);
        bit <<= 1;
    }
    ans
}

#[inline]
fn inv(a: u32) -> u32 {
    pow(a, P - 2)
}

#[inline]
fn choose(n: u32, r: u32) -> u32 {
    let r = r.min(n - r);
    let denominator = (1..=r).fold(1, mul);
    let numerator = (n - r + 1..=n).fold(1, mul);
    mul(numerator, inv(denominator))
}

fn factor(n: u32) -> Vec<(u32, u32)> {
    let mut n = n;
    let mut v: Vec<(u32, u32)> = Vec::new();

    let mut p = 2;
    while p * p <= n {
        if n % p == 0 {
            v.push((p, 0));
        }
        while n % p == 0 {
            v.last_mut().unwrap().1 += 1;
            n /= p;
        }
        p += 1;
    }
    if n > 1 {
        v.push((n, 1));
    }
    v
}

impl Solution {
    pub fn ways_to_fill_array(queries: Vec<Vec<i32>>) -> Vec<i32> {
        queries
            .into_iter()
            .map(|query| {
                let n = query[0] as u32;
                let k = query[1] as u32;
                let factors = factor(k);
                factors
                    .into_iter()
                    .fold(1, |acc, (_, r)| mul(acc, choose(n + r - 1, n - 1)))
                    as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example() {
        let qs = vec![vec![2, 6], vec![5, 1], vec![73, 660]];
        assert_eq!(Solution::ways_to_fill_array(qs), vec![4, 1, 50734910]);

        let qs = vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]];
        assert_eq!(Solution::ways_to_fill_array(qs), vec![1, 2, 3, 10, 5]);
    }
}
