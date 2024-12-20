pub struct Solution;

impl Solution {
    pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let i = nums.iter().position(|&v| v == 1).unwrap();
        let j = nums.iter().position(|&v| v == n as i32).unwrap();

        if i < j {
            (i + n - 1 - j) as i32
        } else {
            (i + n - 2 - j) as i32
        }
    }
}
