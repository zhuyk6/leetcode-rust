pub struct Solution;

impl Solution {
    pub fn sort_the_students(mut score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        use std::cmp::Reverse;
        score.sort_unstable_by_key(|row| Reverse(row[k]));
        score
    }
}
