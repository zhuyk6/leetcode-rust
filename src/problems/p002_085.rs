struct Solution;

use std::collections::HashMap;

#[allow(unused)]
impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        fn get_counter(words: Vec<String>) -> HashMap<String, i32> {
            let mut cnt = HashMap::new();
            for s in words {
                *cnt.entry(s).or_default() += 1;
            }
            cnt
        }

        let cnt1 = get_counter(words1);
        let cnt2 = get_counter(words2);

        cnt1.into_iter()
            .filter(|(k, v)| *v == 1 && cnt2.get(k).map_or(false, |v| *v == 1))
            .count() as i32
    }
}
