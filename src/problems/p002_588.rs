pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let prefix_sum: Vec<i32> = nums
            .iter()
            .scan(0, |s, v| {
                *s ^= v;
                Some(*s)
            })
            .collect();

        let mut ans = 0;
        let mut map: HashMap<i32, u64> = HashMap::new();
        for &sl in prefix_sum.iter().rev() {
            let cnt = map.entry(sl).or_default();
            ans += *cnt;
            *cnt += 1;
        }
        ans += map.get(&0).copied().unwrap_or_default();

        ans as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![4, 3, 1, 2, 4];
        assert_eq!(Solution::beautiful_subarrays(nums), 2);
    }

    #[test]
    fn sample2() {
        let nums = vec![1, 10, 4];
        assert_eq!(Solution::beautiful_subarrays(nums), 0);
    }

    #[test]
    fn sample3() {
        let nums = vec![0, 0];
        assert_eq!(Solution::beautiful_subarrays(nums), 3);

        let nums = vec![0, 0, 0];
        assert_eq!(Solution::beautiful_subarrays(nums), 6);
    }
}
