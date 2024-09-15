struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn clear_digits(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut alive = vec![true; n];

        for (i, c) in s.iter().enumerate() {
            if c.is_ascii_digit() {
                alive[i] = false;
                for (j, cc) in s[..i].iter().enumerate().rev() {
                    if !cc.is_ascii_digit() && alive[j] {
                        alive[j] = false;
                        break;
                    }
                }
            }
        }

        (0..n).filter(|i| alive[*i]).map(|i| s[i]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = "abc".into();
        assert_eq!(Solution::clear_digits(s), "abc".to_string());
    }

    #[test]
    fn sample2() {
        let s = "cb34".into();
        assert_eq!(Solution::clear_digits(s), "".to_string());
    }
}
