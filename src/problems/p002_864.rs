pub struct Solution;

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let n = s.len();
        let m = s.as_bytes().iter().filter(|b| **b == b'1').count();
        let buf = std::iter::repeat('1')
            .take(m - 1)
            .chain(std::iter::repeat('0').take(n - m))
            .chain(std::iter::once('1'));
        String::from_iter(buf)
    }
}

#[test]
fn test1() {
    let s = "010".to_string();
    assert_eq!(Solution::maximum_odd_binary_number(s), "001".to_string());
}
