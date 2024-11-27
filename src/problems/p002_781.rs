pub struct Solution;

#[derive(Debug, Clone, Default)]
struct Node {
    sons: [Option<Box<Node>>; 26],
    end: bool,
}

struct Trie {
    root: Box<Node>,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: Box::new(Node::default()),
        }
    }

    fn insert(&mut self, s: String) {
        let s = s.as_bytes();
        let mut cur = self.root.as_mut();
        for c in s.iter() {
            let c = (*c - b'a') as usize;
            if cur.sons[c].is_none() {
                cur.sons[c] = Some(Box::new(Node::default()));
            }
            cur = cur.sons[c].as_mut().unwrap(); // this must be valid
        }
        cur.end = true;
    }

    fn query(&self, s: &[u8]) -> Option<usize> {
        let mut dep = 0;
        let mut cur = self.root.as_ref();
        for c in s.iter() {
            let c = (*c - b'a') as usize;
            match cur.sons[c].as_deref() {
                Some(next) => {
                    cur = next;
                    dep += 1;
                    if cur.end {
                        return Some(dep);
                    }
                }
                None => return None,
            }
        }
        None
    }
}

impl Solution {
    pub fn longest_valid_substring(word: String, forbidden: Vec<String>) -> i32 {
        let mut tree = Trie::new();
        for s in forbidden {
            tree.insert(s);
        }

        let s = word.as_bytes();
        let n = s.len();

        let arr: Vec<Option<usize>> = (0..n).map(|i| tree.query(&s[i..])).collect();

        // dbg!(&arr);

        use std::collections::VecDeque;

        let mut que = VecDeque::with_capacity(n);

        let mut ans = 0;
        let mut j = 0;
        for i in 0..n {
            while let Some(&(x, _y)) = que.front() {
                if x < i {
                    que.pop_front();
                } else {
                    break;
                }
            }
            j = j.max(i);
            while que.front().is_none() || j < que.front().unwrap().1 {
                let k = arr[j].as_ref().map(|&l| j + l - 1).unwrap_or(n);
                while let Some(&(_x, y)) = que.back() {
                    if y >= k {
                        que.pop_back();
                    } else {
                        break;
                    }
                }
                que.push_back((j, k));

                if j < que.front().unwrap().1 {
                    j += 1;
                } else {
                    break;
                }
            }

            let k = que.front().unwrap().1;
            ans = ans.max(k - i);
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec_owned;

    #[test]
    fn sample1() {
        let word = "cbaaaabc".to_string();
        let forbidden = vec!["aaa".to_string(), "cb".to_string()];
        assert_eq!(Solution::longest_valid_substring(word, forbidden), 4);
    }

    #[test]
    fn sample2() {
        let word = "leetcode".to_string();
        let forbidden = vec!["de".to_string(), "le".to_string(), "e".to_string()];
        assert_eq!(Solution::longest_valid_substring(word, forbidden), 4);
    }

    #[test]
    fn issue() {
        let word = "bcac".to_string();
        let forbidden = nested_vec_owned!["bcac", "caca", "bcac", "bca"];
        assert_eq!(Solution::longest_valid_substring(word, forbidden), 3);
    }
}
