pub struct Solution;

impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let s = s.as_bytes();
        let mut f = 0;
        let mut num1 = if s[0] == b'0' { 0 } else { 1 };

        for &v in &s[1..] {
            if v == b'1' {
                num1 += 1;
            } else {
                f += num1;
            }
        }

        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = "101".to_string();
        assert_eq!(Solution::minimum_steps(s), 1);
    }

    #[test]
    fn sample2() {
        let s = "100".to_string();
        assert_eq!(Solution::minimum_steps(s), 2);
    }

    #[test]
    fn sample3() {
        let s = "0111".to_string();
        assert_eq!(Solution::minimum_steps(s), 0);
    }
}
