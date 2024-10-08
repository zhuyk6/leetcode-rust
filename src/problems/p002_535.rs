struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let s1: i32 = nums.iter().sum();
        let s2: i32 = nums
            .iter()
            .map(|&x| {
                debug_assert!(x > 0);
                let mut s = 0;
                let mut x = x;
                while x > 0 {
                    s += x % 10;
                    x /= 10;
                }
                s
            })
            .sum();
        (s1 - s2).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![1, 15, 6, 3];
        assert_eq!(Solution::difference_of_sum(nums), 9);
    }

    #[test]
    fn sample2() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::difference_of_sum(nums), 0);
    }
}
