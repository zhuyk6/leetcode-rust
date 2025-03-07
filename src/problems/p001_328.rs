pub struct Solution;

impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        let mut s = palindrome.into_bytes();

        let n = s.len();
        if n <= 1 {
            return String::new();
        }
        let mid = if n % 2 == 1 { n / 2 } else { n };

        for (i, c) in s.iter_mut().enumerate() {
            if i != mid && *c != b'a' {
                *c = b'a';
                return String::from_utf8(s).unwrap();
            }
        }

        if let Some(c) = s.last_mut() {
            *c = b'b';
        }
        String::from_utf8(s).unwrap()
    }
}
