struct Solution;

const MOD: i64 = 1_000_000_007;

#[allow(unused)]
impl Solution {
    pub fn minimum_possible_sum(n: i32, target: i32) -> i32 {
        let n = n as i64;
        let t = target as i64;

        let m = t / 2;
        (if n <= m {
            (1 + n) * n / 2 % MOD
        } else {
            let s1 = (1 + m) * m / 2 % MOD;
            let n = n - m;
            let s2 = (t + t + n - 1) * n / 2 % MOD;
            (s1 + s2) % MOD
        }) as i32
    }
}

#[test]
fn test1() {
    let n = 2;
    let t = 3;
    assert_eq!(Solution::minimum_possible_sum(n, t), 4);
}

#[test]
fn test2() {
    let n = 3;
    let t = 3;
    assert_eq!(Solution::minimum_possible_sum(n, t), 8);
}

#[test]
fn test3() {
    let n = 1;
    let t = 1;
    assert_eq!(Solution::minimum_possible_sum(n, t), 1);
}
