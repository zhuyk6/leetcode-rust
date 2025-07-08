pub struct Solution;

const MOD: i32 = 1_000_000_007;

fn mul_mod(a: i32, b: i32) -> i32 {
    ((a as i64 * b as i64) % MOD as i64) as i32
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

impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let nums = nums;

        if nums[0] * 2 > target {
            return 0;
        }

        let mut ans = 0;
        let n = nums.len();
        let mut j = n - 1;
        let mut i = 0;
        while i <= j {
            while i <= j && nums[i] + nums[j] > target {
                j -= 1;
            }
            if i > j {
                break;
            }
            let len = j - i;
            ans = (ans + pow_mod(2, len as u32)) % MOD;

            i += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![3, 5, 6, 7];
        let target = 9;
        assert_eq!(Solution::num_subseq(nums, target), 4);
    }

    #[test]
    fn sample2() {
        let nums = vec![3, 3, 6, 8];
        let target = 10;
        assert_eq!(Solution::num_subseq(nums, target), 6);
    }

    #[test]
    fn sample3() {
        let nums = vec![2, 3, 3, 4, 6, 7];
        let target = 12;
        assert_eq!(Solution::num_subseq(nums, target), 61);
    }

    #[test]
    fn sample4() {
        let nums = vec![1];
        let target = 1;
        assert_eq!(Solution::num_subseq(nums, target), 0);
    }

    #[test]
    fn sample5() {
        let nums = vec![1, 5];
        let target = 2;
        assert_eq!(Solution::num_subseq(nums, target), 1);
    }

    #[test]
    fn issue() {
        let nums = vec![
            14, 4, 6, 6, 20, 8, 5, 6, 8, 12, 6, 10, 14, 9, 17, 16, 9, 7, 14, 11, 14, 15, 13, 11,
            10, 18, 13, 17, 17, 14, 17, 7, 9, 5, 10, 13, 8, 5, 18, 20, 7, 5, 5, 15, 19, 14,
        ];
        let target = 22;
        assert_eq!(Solution::num_subseq(nums, target), 272187084);
    }
}
