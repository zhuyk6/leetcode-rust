pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let n = nums.len();
        let mut ans = 0i64;

        let mut j = 0;
        let mut acc = nums[0] as i64;
        for i in 0..n {
            if j < i {
                j = i;
                acc = nums[i] as i64;
            }
            while j < n && acc * ((j - i + 1) as i64) < k {
                j += 1;
                if j < n {
                    acc += nums[j] as i64;
                }
            }
            ans += (j - i) as i64;
            acc -= nums[i] as i64;
        }

        ans
    }
}
