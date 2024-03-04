struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_value_after_reverse(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let v = |i: usize| -> i32 {
            assert!(i > 0);
            (nums[i] - nums[i - 1]).abs()
        };

        let sum_v: i32 = (1..n).map(v).sum();

        let mut ans = 0;

        // reverse [0, i]
        for i in 1..(n - 1) {
            ans = ans.max((nums[i + 1] - nums[0]).abs() - v(i + 1));
        }

        // reverse [i, n-1]
        for i in 1..(n - 1) {
            ans = ans.max((nums[n - 1] - nums[i - 1]).abs() - v(i));
        }

        // reverse [l, r]
        // 4 cases
        let mut max = -nums[1] - nums[0] - v(1);
        for i in 2..(n - 1) {
            ans = ans.max(nums[i + 1] + nums[i] - v(i + 1) + max);
            max = max.max(-nums[i] - nums[i - 1] - v(i));
        }

        let mut max = nums[1] - nums[0] - v(1);
        for i in 1..(n - 1) {
            ans = ans.max(-nums[i + 1] + nums[i] - v(i + 1) + max);
            max = max.max(nums[i] - nums[i - 1] - v(i));
        }

        let mut max = -nums[1] + nums[0] - v(1);
        for i in 1..(n - 1) {
            ans = ans.max(nums[i + 1] - nums[i] - v(i + 1) + max);
            max = max.max(-nums[i] + nums[i - 1] - v(i));
        }

        let mut max = nums[1] + nums[0] - v(1);
        for i in 1..(n - 1) {
            ans = ans.max(-nums[i + 1] - nums[i] - v(i + 1) + max);
            max = max.max(nums[i] + nums[i - 1] - v(i));
        }

        sum_v + ans
    }
}

#[test]
fn test1() {
    let nums = vec![2, 3, 1, 5, 4];
    assert_eq!(Solution::max_value_after_reverse(nums), 10);
}

#[test]
fn test2() {
    let nums = vec![2, 4, 9, 24, 2, 1, 10];
    assert_eq!(Solution::max_value_after_reverse(nums), 68);
}
