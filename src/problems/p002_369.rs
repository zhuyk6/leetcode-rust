struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut f = vec![false; n];
        f[1] = nums[1] == nums[0];
        if n == 2 {
            return f[1];
        }
        f[2] = (nums[0] == nums[1]) && (nums[1] == nums[2])
            || (nums[1] - nums[0] == 1) && (nums[2] - nums[1] == 1);
        for i in 3..n {
            f[i] = (nums[i] == nums[i - 1]) && f[i - 2]
                || (nums[i] == nums[i - 1]) && (nums[i - 1] == nums[i - 2]) && f[i - 3]
                || (nums[i] - nums[i - 1] == 1) && (nums[i - 1] - nums[i - 2] == 1) && f[i - 3];
        }
        f[n - 1]
    }
}

#[test]
fn test1() {
    let nums = vec![4, 4, 4, 5, 6];
    assert!(Solution::valid_partition(nums));
}

#[test]
fn test2() {
    let nums = vec![1, 1, 1, 2];
    assert!(!Solution::valid_partition(nums));
}
