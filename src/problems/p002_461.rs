pub struct Solution;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;
        use std::collections::HashMap;

        let mut set: HashMap<i32, u32> = HashMap::new();

        let mut num_multiples = 0;
        let mut acc: i64 = 0;
        // 0..k
        for &v in &nums[..k] {
            let e = set.entry(v).or_default();
            *e += 1;
            if *e == 2 {
                num_multiples += 1;
            }
            acc += v as i64;
        }

        let mut ans = if num_multiples == 0 { acc } else { 0 };

        for i in k..n {
            // add nums[i]
            let e = set.entry(nums[i]).or_default();
            *e += 1;
            if *e == 2 {
                num_multiples += 1;
            }
            acc += nums[i] as i64;

            // remove nums[i - k]
            let c = set.entry(nums[i - k]).or_default();
            *c -= 1;
            if *c == 1 {
                num_multiples -= 1;
            }
            acc -= nums[i - k] as i64;

            if num_multiples == 0 {
                ans = ans.max(acc);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![1, 5, 4, 2, 9, 9, 9];
        let k = 3;
        assert_eq!(Solution::maximum_subarray_sum(nums, k), 15);
    }

    #[test]
    fn sample2() {
        let nums = vec![4, 4, 4];
        let k = 3;
        assert_eq!(Solution::maximum_subarray_sum(nums, k), 0);
    }
}
