pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    fn calc1(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();

        let mut ans = 0;
        let mut i = 0;

        while i < n {
            if nums[i] == k {
                let mut j = i;
                while j + 1 < n && nums[j + 1] == k {
                    j += 1;
                }
                let len = (j - i + 1) as i64;
                ans += len * (len + 1) / 2;
                i = j + 1;
            } else {
                i += 1;
            }
        }

        ans
    }

    fn calc2(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        assert!(min_k < max_k);

        let n = nums.len();
        let mut ans = 0;

        let out_bound = |v: i32| v < min_k || max_k < v;

        let mut next_bound: Vec<Option<usize>> = vec![None; n];
        let mut next_out: Vec<Option<usize>> = vec![None; n];

        let mut out_pos = None;
        let mut lower_bound_pos = None;
        let mut upper_bound_pos = None;

        for (i, &v) in nums.iter().enumerate().rev() {
            if v == min_k {
                next_bound[i] = upper_bound_pos;
                next_out[i] = out_pos;
                lower_bound_pos = Some(i);
            } else if v == max_k {
                next_bound[i] = lower_bound_pos;
                next_out[i] = out_pos;
                upper_bound_pos = Some(i);
            } else if out_bound(v) {
                out_pos = Some(i);
            }
        }

        let mut prev_pos: Option<usize> = None;
        for (i, &v) in nums.iter().enumerate() {
            if v == min_k || v == max_k {
                let len1 = match prev_pos {
                    Some(pos) => i - pos,
                    None => i + 1,
                };
                prev_pos = Some(i);

                let len2 = match (next_bound[i], next_out[i]) {
                    (Some(bound), Some(out)) => out.saturating_sub(bound),
                    (Some(bound), None) => n - bound,
                    (None, Some(..)) => 0,
                    (None, None) => 0,
                };

                ans += len1 as i64 * len2 as i64;
            } else if out_bound(v) {
                prev_pos = Some(i);
            }
        }

        ans
    }

    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        match min_k.cmp(&max_k) {
            Ordering::Less => Self::calc2(nums, min_k, max_k),
            Ordering::Equal => Self::calc1(nums, min_k),
            Ordering::Greater => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![1, 3, 5, 2, 7, 5];
        let min_k = 1;
        let max_k = 5;
        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), 2);
    }

    #[test]
    fn sample2() {
        let nums = vec![1, 1, 1, 1];
        let min_k = 1;
        let max_k = 1;
        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), 10);
    }
}
