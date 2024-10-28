pub struct Solution;

use std::iter::{once, repeat};

impl Solution {
    pub fn maximum_binary_string(s: String) -> String {
        let tot = s.len();
        let mut iter = s.as_bytes().iter();
        let n1 = iter.position(|c| *c == b'0');

        if let Some(n1) = n1 {
            let n = tot - n1;
            let m = 1 + iter.filter(|c| **c == b'0').count();

            String::from_iter(
                repeat('1')
                    .take(n1 + m - 1)
                    .chain(once('0'))
                    .chain(repeat('1').take(n - m)),
            )
        } else {
            s
        }
    }
}

#[test]
fn test1() {
    let s = "1100".to_string();
    assert_eq!(Solution::maximum_binary_string(s), "1110".to_string());
}
