pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut first = HashMap::new();

        let mut ans = true;
        s.chars().enumerate().for_each(|(pos, c)| {
            if let Some(q) = first.get(&c) {
                if pos - q - 1 != distance[(c as u8 - b'a') as usize] as usize {
                    ans = false;
                }
            } else {
                first.insert(c, pos);
            }
        });
        ans
    }
}
