pub struct Solution;

impl Solution {
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        let (acc1, acc2) = nums.into_iter().fold((0, 0), |(acc1, acc2), v| {
            if v < 10 {
                (acc1 + v, acc2)
            } else {
                (acc1, acc2 + v)
            }
        });
        acc1 != acc2
    }
}
