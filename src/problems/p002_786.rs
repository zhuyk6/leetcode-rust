struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_score(nums: Vec<i32>, x: i32) -> i64 {
        let mut odd = 0;
        let mut even = 0;

        for &v in nums.iter().rev() {
            match v % 2 {
                0 => {
                    even = v as i64 + i64::max(odd - x as i64, even);
                }
                1 => {
                    odd = v as i64 + i64::max(even - x as i64, odd);
                }
                _ => panic!("impossible"),
            }
        }

        match nums[0] % 2 {
            0 => even,
            1 => odd,
            _ => panic!("impossible"),
        }
    }
}
