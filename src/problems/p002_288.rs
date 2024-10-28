pub struct Solution;

impl Solution {
    pub fn discount_prices(sentence: String, discount: i32) -> String {
        let f = |str: &str| -> String {
            if let Some(suffix) = str.strip_prefix('$') {
                if suffix.contains('e') {
                    str.to_string()
                } else {
                    match suffix.parse::<f64>() {
                        Ok(v) if v.signum() >= 0.0 => {
                            let u = v * (100 - discount) as f64 / 100.0;
                            format!("${:.2}", u)
                        }
                        _ => str.to_string(),
                    }
                }
            } else {
                str.to_string()
            }
        };

        sentence.split(' ').map(f).collect::<Vec<_>>().join(" ")
    }
}

#[test]
fn wrong1() {
    let s = "$1e9".to_string();
    let discount = 50;

    assert_eq!(Solution::discount_prices(s, discount), "$1e9".to_string());
}
