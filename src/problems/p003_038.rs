pub struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        assert!(nums.len() >= 2);

        let s = nums[0] + nums[1];
        nums.chunks(2)
            .take_while(|&w| w.len() == 2 && w[0] + w[1] == s)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![3, 2, 1, 4, 5];
        assert_eq!(Solution::max_operations(nums), 2);
    }

    #[test]
    fn sample2() {
        let nums = vec![3, 2, 6, 1, 4];
        assert_eq!(Solution::max_operations(nums), 1);
    }
}
