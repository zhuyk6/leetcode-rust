struct Solution;

const PRIMES: [i32; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
const MAX_VAL: usize = 30;

const MOD: i64 = 1_000_000_007;

fn pow(mut a: i64, n: i64) -> i64 {
    let mut acc = 1;
    let mut bit = 1;
    while bit <= n {
        if n & bit > 0 {
            acc = (acc * a) % MOD;
        }
        a = (a * a) % MOD;
        bit <<= 1;
    }
    acc
}

#[allow(dead_code, clippy::needless_range_loop)]
impl Solution {
    pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
        let mut frequency = [0; MAX_VAL + 1];
        for v in &nums {
            let v = *v as usize;
            frequency[v] += 1;
        }

        let mut can_select = [true; MAX_VAL + 1];
        let mut mask = [0; MAX_VAL + 1];
        for v in 2..=MAX_VAL {
            for p in PRIMES {
                if v as i32 % (p * p) == 0 {
                    can_select[v] = false;
                }
            }
            if can_select[v] {
                for (i, &p) in PRIMES.iter().enumerate() {
                    if v as i32 % p == 0 {
                        mask[v] |= 1 << i;
                    }
                }
            }
        }

        let mut f = vec![vec![0i64; 1 << PRIMES.len()]; MAX_VAL + 1];

        // f[1][0] = 1 << frequency[1];
        f[1][0] = pow(2, frequency[1]);

        for i in 2..=MAX_VAL {
            for m in 0..(1 << PRIMES.len()) {
                f[i][m] = f[i - 1][m];
                if can_select[i] && m | mask[i] == m {
                    f[i][m] = (f[i][m] + f[i - 1][m ^ mask[i]] * frequency[i] % MOD) % MOD;
                }
                // if f[i][m] > 0 {
                //     println!("f[{i}][{m:10b}] = {}", f[i][m]);
                // }
            }
        }

        let mut ans = 0;
        for m in 1..(1 << PRIMES.len()) {
            ans = (ans + f[MAX_VAL][m]) % MOD;
        }
        ans as i32
    }
}

#[test]
fn test1() {
    let nums = vec![1, 2, 3, 4];
    assert_eq!(Solution::number_of_good_subsets(nums), 6);
}

#[test]
fn test2() {
    let nums = vec![4, 2, 3, 15];
    assert_eq!(Solution::number_of_good_subsets(nums), 5);
}

#[test]
fn test3() {
    let nums = vec![2, 3, 5, 15];
    assert_eq!(Solution::number_of_good_subsets(nums), 9);
}

#[test]
fn test4() {
    let nums = vec![
        10, 11, 5, 1, 10, 1, 3, 1, 26, 11, 6, 1, 1, 15, 1, 7, 22, 1, 1, 1, 1, 1, 23, 1, 29, 5, 6,
        1, 1, 29, 1, 1, 21, 19, 1, 1, 1, 2, 1, 11, 1, 15, 1, 22, 14, 1, 1, 1, 1, 6, 7, 1, 14, 3, 5,
        1, 22, 1, 1, 1, 17, 1, 29, 2, 1, 15, 10, 1, 5, 7, 1, 1, 1, 30, 1, 30, 1, 21, 10, 1, 1, 1,
        1, 1, 2, 6, 5, 7, 3, 1, 1, 19, 29, 1, 7, 13, 14, 1, 5, 26, 19, 11, 1, 1, 1, 1, 1, 1, 1, 1,
        22, 15, 1, 1, 13, 1, 17, 1, 1, 1, 13, 6, 1, 10, 1, 1, 17, 1, 1, 3, 14, 7, 17, 1, 13, 1, 1,
        1, 1, 1, 11, 1, 1, 6, 1, 1, 1, 1, 1, 2, 1, 30, 2, 26, 1, 1, 14, 1, 26, 29, 30, 1, 13, 21,
        1, 1, 14, 21, 1, 23, 1, 15, 23, 21, 1, 30, 19, 19, 1, 10, 23, 3, 3, 17, 22, 2, 26, 1, 11,
        1, 23, 1, 1, 1, 15, 1, 1, 13, 1, 1,
    ];
    assert_eq!(Solution::number_of_good_subsets(nums), 520317213);
}
