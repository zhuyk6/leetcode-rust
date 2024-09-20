struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_stars(s: String) -> String {
        let n = s.len();
        let mut keep = vec![true; n];

        let mut num_delete = 0;
        for (i, c) in s.bytes().enumerate().rev() {
            if c == b'*' {
                num_delete += 1;
                keep[i] = false;
            } else if num_delete > 0 {
                num_delete -= 1;
                keep[i] = false;
            }
        }
        unsafe {
            String::from_utf8_unchecked(
                s.bytes()
                    .enumerate()
                    .filter(|&(i, _)| keep[i])
                    .map(|(_, c)| c)
                    .collect(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = "leet**cod*e".to_string();
        assert_eq!(Solution::remove_stars(s), "lecoe".to_string());
    }

    #[test]
    fn sample2() {
        let s = "erase*****".to_string();
        assert_eq!(Solution::remove_stars(s), "".to_string());
    }
}
