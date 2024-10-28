pub struct Solution;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let n = candy_type.len();
        let set: HashSet<i32> = HashSet::from_iter(candy_type);
        let m = set.len();

        usize::min(n / 2, m) as i32
    }
}
