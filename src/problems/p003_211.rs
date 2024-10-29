pub struct Solution;

impl Solution {
    pub fn valid_strings(n: i32) -> Vec<String> {
        match n {
            0 => vec!["".into()],
            n => {
                let v = Solution::valid_strings(n - 1);
                let mut ret = vec![];
                for s in v {
                    let mut s = s.into_bytes();
                    // insert 1
                    s.push(b'1');
                    ret.push(unsafe { String::from_utf8_unchecked(s.clone()) });
                    s.pop();
                    // insert 0
                    if let Some(&c) = s.last() {
                        if c == b'1' {
                            s.push(b'0');
                            ret.push(unsafe { String::from_utf8_unchecked(s) });
                        }
                    } else {
                        s.push(b'0');
                        ret.push(unsafe { String::from_utf8_unchecked(s) });
                    }
                }
                ret
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let n = 3;
        let left = ["010", "011", "101", "110", "111"];
        let mut left: Vec<String> = left.into_iter().map(|s| s.into()).collect();
        left.sort();

        let mut right = Solution::valid_strings(n);
        right.sort();

        assert_eq!(left, right);
    }

    #[test]
    fn sample2() {
        let n = 1;
        let left = ["0", "1"];
        let mut left: Vec<String> = left.into_iter().map(|s| s.into()).collect();
        left.sort();

        let mut right = Solution::valid_strings(n);
        right.sort();

        assert_eq!(left, right);
    }
}
