pub struct Solution;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        use std::collections::HashSet;

        let mut ans_set: HashSet<String> = HashSet::new();

        for edge in &paths {
            ans_set.insert(edge[1].clone());
        }

        for edge in &paths {
            ans_set.take(&edge[0]);
        }

        ans_set
            .into_iter()
            .next()
            .expect("There should be at least one solution!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::{nested_vec, nested_vec_owned};

    #[test]
    fn sample1() {
        let paths = nested_vec![
            ["London".to_string(), "New York".to_string()],
            ["New York".to_string(), "Lima".to_string()],
            ["Lima".to_string(), "Sao Paulo".to_string()],
        ];
        assert_eq!(Solution::dest_city(paths), "Sao Paulo".to_string());
    }

    #[test]
    fn sample2() {
        let paths = nested_vec_owned![["B", "C"], ["D", "B"], ["C", "A"]];
        assert_eq!(Solution::dest_city(paths), "A".to_string());
    }
}
