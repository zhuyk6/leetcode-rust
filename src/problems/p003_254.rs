pub struct Solution;

impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;

        let mut ans = vec![0; n - k + 1];

        let mut j = 0;
        for i in 0..(n - k + 1) {
            while j < n && nums[j] - nums[i] == (j - i) as i32 {
                j += 1;
            }
            ans[i] = if j - i >= k { nums[i + k - 1] } else { -1 };
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![1, 2, 3, 4, 3, 2, 5];
        let k = 3;
        assert_eq!(Solution::results_array(nums, k), vec![3, 4, -1, -1, -1]);
    }

    #[test]
    fn sample2() {
        let nums = vec![2, 2, 2, 2, 2];
        let k = 4;
        assert_eq!(Solution::results_array(nums, k), vec![-1, -1]);
    }

    #[test]
    fn sample3() {
        let nums = vec![3, 2, 3, 2, 3, 2];
        let k = 2;
        assert_eq!(Solution::results_array(nums, k), vec![-1, 3, -1, 3, -1]);
    }
}
