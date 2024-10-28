pub struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();

        nums.sort_unstable();
        nums.dedup();

        let m = nums.len();
        let mut ans = n;

        let mut r = 0;
        for l in 0..m {
            while r < m && nums[r] - nums[l] < n as i32 {
                r += 1;
            }
            ans = ans.min(n - (r - l));
        }

        ans as i32
    }
}

#[test]
fn test1() {
    let nums = vec![4, 2, 5, 3];
    assert_eq!(Solution::min_operations(nums), 0);
}

#[test]
fn test2() {
    let nums = vec![1, 2, 3, 5, 6];
    assert_eq!(Solution::min_operations(nums), 1);
}

#[test]
fn test3() {
    let nums = vec![1, 10, 100, 1000];
    assert_eq!(Solution::min_operations(nums), 3);
}
