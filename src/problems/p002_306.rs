struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        // use std::collections::HashSet;
        // use std::collections::BTreeSet;
        use std::{
            collections::HashSet,
            hash::{DefaultHasher, Hash, Hasher},
        };

        // let mut leading: [HashSet<String>; 26] = Default::default();
        // let mut leading: [BTreeSet<String>; 26] = Default::default();
        let mut leading: [HashSet<u64>; 26] = Default::default();

        for idea in &ideas {
            let v = idea.as_bytes();
            let c = (v[0] - b'a') as usize;
            // let post = unsafe { String::from_utf8_unchecked(v[1..].to_owned()) };

            let mut hasher = DefaultHasher::default();
            // post.hash(&mut hasher);
            v[1..].hash(&mut hasher);
            leading[c].insert(hasher.finish());
        }

        let mut ans = 0;
        for i in 0..26 {
            for j in (i + 1)..26 {
                let s1 = &leading[i];
                let s2 = &leading[j];
                let c = s1.intersection(s2).count();
                ans += 2 * (s1.len() - c) * (s2.len() - c);
            }
        }

        ans as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec_owned;

    #[test]
    fn sample1() {
        let ideas = nested_vec_owned!["coffee", "donuts", "time", "toffee"];
        assert_eq!(Solution::distinct_names(ideas), 6);
    }

    #[test]
    fn sample2() {
        let ideas = nested_vec_owned!["lack", "back"];
        assert_eq!(Solution::distinct_names(ideas), 0);
    }
}
