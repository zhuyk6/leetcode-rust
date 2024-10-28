pub struct Solution;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut stack: Vec<usize> = Vec::new();
        let mut delete: Vec<bool> = vec![false; s.len()];

        for (i, b) in s.as_bytes().iter().enumerate() {
            if *b == b'(' {
                stack.push(i);
            } else if *b == b')' {
                if stack.is_empty() {
                    delete[i] = true;
                } else {
                    stack.pop();
                }
            }
        }

        while let Some(i) = stack.pop() {
            delete[i] = true;
        }

        let mut t = String::new();
        for (i, b) in s.as_bytes().iter().enumerate() {
            if !delete[i] {
                t.push(*b as char);
            }
        }
        t
    }
}

#[test]
fn test1() {
    let s = "lee(t(c)o)de)".into();
    assert_eq!(
        Solution::min_remove_to_make_valid(s),
        "lee(t(c)o)de".to_string()
    );
}

#[test]
fn test2() {
    let s = "))((".into();
    assert_eq!(Solution::min_remove_to_make_valid(s), "".to_string());
}
