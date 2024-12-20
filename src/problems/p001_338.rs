pub struct Solution;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        use std::collections::HashMap;
        let mut cnt = HashMap::new();
        for &num in arr.iter() {
            *cnt.entry(num).or_insert(0) += 1;
        }
        let mut cnt: Vec<(i32, usize)> = cnt.into_iter().collect();
        cnt.sort_unstable_by_key(|(_, c)| *c);

        let mut ans = 0;
        let mut acc = 0;
        for (_, c) in cnt.iter().rev() {
            ans += 1;
            acc += c;
            if acc * 2 >= n {
                break;
            }
        }
        ans
    }
}
