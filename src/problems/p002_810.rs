pub struct Solution;

impl Solution {
    pub fn final_string(s: String) -> String {
        let mut t = String::new();
        for c in s.chars() {
            match c {
                'i' => t = t.chars().rev().collect(),
                _ => t.push(c),
            }
        }
        t
    }
}

#[test]
fn test1() {
    let s = "poiinter".to_string();
    assert_eq!(Solution::final_string(s), "ponter".to_string());
}

#[test]
fn test2() {
    let s = "viwif".to_string();
    assert_eq!(Solution::final_string(s), "wvf".to_string());
}
