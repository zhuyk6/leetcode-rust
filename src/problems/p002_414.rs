pub struct Solution;

impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        assert!(n >= 1);

        let mut i = 0;
        let mut ans = 0;
        while i < n {
            let mut j = i + 1;
            while j < n && s[j] + i as u8 == s[i] + j as u8 {
                j += 1;
            }
            ans = ans.max(j - i);
            i = j;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = "abacaba".to_string();
        assert_eq!(Solution::longest_continuous_substring(s), 2);
    }

    #[test]
    fn sample2() {
        let s = "abcde".to_string();
        assert_eq!(Solution::longest_continuous_substring(s), 5);
    }
}
