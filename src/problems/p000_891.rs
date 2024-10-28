pub struct Solution;

const MOD: i64 = 1_000_000_007;

fn pow(mut a: i64, n: i64) -> i64 {
    let mut acc = 1;
    let mut bit = 1;
    while bit <= n {
        if n & bit > 0 {
            acc = acc * a % MOD;
        }
        a = a * a % MOD;
        bit <<= 1;
    }
    acc
}

fn inv(x: i64) -> i64 {
    pow(x, MOD - 2)
}

impl Solution {
    pub fn sum_subseq_widths(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort_unstable();

        let mut pow2 = vec![0i64; n];
        pow2[0] = 1;
        for i in 1..n {
            pow2[i] = pow2[i - 1] * 2 % MOD;
        }

        let inv2: i64 = inv(2);
        let mut ipow2 = vec![0i64; n];
        ipow2[0] = 1;
        for i in 1..n {
            ipow2[i] = ipow2[i - 1] * inv2 % MOD;
        }

        let s1 = {
            let mut acc: i64 = 0;
            for &v in &nums {
                acc = (acc + v as i64) % MOD;
            }
            acc
        };

        let s2 = {
            let mut acc: i64 = 0;
            for (i, &v) in nums.iter().enumerate() {
                acc = (acc + v as i64 * pow2[n - 1 - i] % MOD) % MOD;
            }
            acc
        };

        let s3 = {
            let mut acc: i64 = 0;
            let mut suffix: i64 = 0;
            for i in (1..n).rev() {
                suffix = (suffix + nums[i] as i64 * pow2[i] % MOD) % MOD;
                acc = (acc + suffix * ipow2[i] % MOD) % MOD;
            }
            acc
        };

        (((s1 + (MOD - s2)) % MOD + s3) % MOD) as i32
    }
}

#[test]
fn test1() {
    let nums = vec![2, 1, 3];
    assert_eq!(Solution::sum_subseq_widths(nums), 6);
}

#[test]
fn test2() {
    let nums = vec![2];
    assert_eq!(Solution::sum_subseq_widths(nums), 0);
}
