struct Solution;

#[allow(unused)]
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut ans = nums[k as usize];
        let mut min = nums[k as usize];
        let mut i = k as usize;
        let mut j = k as usize;

        while i > 0 && j < n - 1 {
            if nums[i - 1] > nums[j + 1] {
                i -= 1;
                min = min.min(nums[i]);
                ans = ans.max((j - i + 1) as i32 * min);
            } else {
                j += 1;
                min = min.min(nums[j]);
                ans = ans.max((j - i + 1) as i32 * min);
            }
            println!("i = {i}, j = {j}, min = {min}, ans = {ans}");
        }

        while i > 0 {
            i -= 1;
            min = min.min(nums[i]);
            ans = ans.max((j - i + 1) as i32 * min);
        }
        while j < n - 1 {
            j += 1;
            min = min.min(nums[j]);
            ans = ans.max((j - i + 1) as i32 * min);
        }

        ans
    }
}

#[test]
fn test1() {
    let nums = vec![1, 4, 3, 7, 4, 5];
    let k = 3;
    assert_eq!(Solution::maximum_score(nums, k), 15);
}

#[test]
fn test2() {
    let nums = vec![5, 5, 4, 5, 4, 1, 1, 1];
    let k = 0;
    assert_eq!(Solution::maximum_score(nums, k), 20);
}
