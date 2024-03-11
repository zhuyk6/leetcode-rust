struct Solution;

#[allow(unused)]
impl Solution {
    pub fn capitalize_title(title: String) -> String {
        title
            .split_ascii_whitespace()
            .map(|s| {
                if s.len() <= 2 {
                    s.to_uppercase()
                } else {
                    let mut t = s.to_lowercase();
                    t[..1].make_ascii_uppercase();
                    t
                }
            })
            .reduce(|acc, e| format!("{acc} {e}"))
            .unwrap()
    }
}
