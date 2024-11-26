pub struct Solution;

const MOD: i32 = 1_000_000_007;

#[inline]
fn mul(x: i32, y: i32) -> i32 {
    ((x as i64) * (y as i64) % (MOD as i64)) as i32
}

#[inline]
fn add(x: i32, y: i32) -> i32 {
    (x + y) % MOD
}

impl Solution {
    pub fn sum_of_power(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let n = nums.len();
        let mut ans = 0;
        let mut suffix_sum = 0;

        for i in (0..n).rev() {
            // only nums[i]
            ans = add(ans, mul(nums[i], mul(nums[i], nums[i])));

            // nums[i] + suffix_sum
            ans = add(ans, mul(nums[i], suffix_sum));

            // update suffix_sum
            suffix_sum = add(mul(2, suffix_sum), mul(nums[i], nums[i]));
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![2, 1, 4];
        assert_eq!(Solution::sum_of_power(nums), 141);
    }

    #[test]
    fn sample2() {
        let nums = vec![1, 1, 1];
        assert_eq!(Solution::sum_of_power(nums), 7);
    }
}
