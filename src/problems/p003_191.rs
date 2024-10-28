pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        assert!(n >= 3);

        // let mut dp = vec![[i32::MAX; 8]; n];
        let mut dp = [[i32::MAX; 8]; 2];
        let mut pre = 0;
        let mut cur = 1;

        // initialize
        let tmp = ((nums[0] << 2) + (nums[1] << 1) + nums[2]) as usize;
        dp[cur][tmp] = 0;
        dp[cur][(!tmp) & 0b111] = 1;

        // loop
        for &v in &nums[3..] {
            std::mem::swap(&mut cur, &mut pre);
            for j in 0..8 {
                if j & 1 == v as usize {
                    let k = (j >> 1) + 0b100;
                    dp[cur][j] = dp[pre][k];
                } else {
                    let k = ((!(j >> 1)) & 0b11) + 0b100;
                    dp[cur][j] = dp[pre][k].saturating_add(1);
                }
            }
        }
        match dp[cur][0b111] {
            i32::MAX => -1,
            v => v,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![0, 1, 1, 1, 0, 0];
        assert_eq!(Solution::min_operations(nums), 3);
    }

    #[test]
    fn sample2() {
        let nums = vec![0, 1, 1, 1];
        assert_eq!(Solution::min_operations(nums), -1);
    }
}
