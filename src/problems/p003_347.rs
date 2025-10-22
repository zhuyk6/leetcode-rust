pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        nums.sort_unstable();

        let mut counter: HashMap<i32, i32> = HashMap::new();
        for &v in &nums {
            *counter.entry(v).or_default() += 1;
        }

        // calculate when `mid` is the selected value,
        // the maximum frequency can be achieved.
        let calc = |mid: i32| -> i32 {
            let l = nums.partition_point(|&val| val + k < mid);
            let r = nums.partition_point(|&val| val - k <= mid);

            let cnt = counter.get(&mid).cloned().unwrap_or(0);

            i32::min(num_operations + cnt, (r - l) as i32)
        };

        let mut ans = 0;

        for &v in &nums {
            ans = ans.max(calc(v - k));
            ans = ans.max(calc(v));
            ans = ans.max(calc(v + k));
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![1, 4, 5];
        let k = 1;
        let num_operations = 2;
        assert_eq!(Solution::max_frequency(nums, k, num_operations), 2);
    }

    #[test]
    fn sample2() {
        let nums = vec![5, 11, 20, 20];
        let k = 5;
        let num_operations = 1;
        assert_eq!(Solution::max_frequency(nums, k, num_operations), 2);
    }
}
