pub struct Solution;

#[derive(Debug)]
struct Node {
    sons: [usize; 26],
    fail: usize,
    is_end: bool,
    dep: usize,
}

impl Node {
    fn new(dep: usize) -> Self {
        Self {
            sons: [usize::MAX; 26],
            is_end: false,
            fail: usize::MAX,
            dep,
        }
    }
}

#[derive(Debug)]
struct Trie {
    nodes: Vec<Node>,
}

impl Trie {
    fn new() -> Self {
        Self {
            nodes: vec![Node::new(0)],
        }
    }

    // fn with_capacity(capacity: usize) -> Self {
    //     let mut nodes = Vec::with_capacity(capacity);
    //     nodes.push(Node::new());
    //     Self { nodes }
    // }

    fn insert(&mut self, word: &str) {
        let mut cur = 0;
        for (&c, dep) in word.as_bytes().iter().zip(1..) {
            let idx = (c - b'a') as usize;
            if self.nodes[cur].sons[idx] == usize::MAX {
                self.nodes.push(Node::new(dep));
                self.nodes[cur].sons[idx] = self.nodes.len() - 1;
            }
            cur = self.nodes[cur].sons[idx];
        }
        self.nodes[cur].is_end = true;
    }

    fn build(&mut self) {
        let mut queue = std::collections::VecDeque::new();
        self.nodes[0].fail = 0;

        for idx in 0..26 {
            let x = self.nodes[0].sons[idx];
            if x < usize::MAX {
                self.nodes[x].fail = 0;
                queue.push_back(x);
            }
        }

        while let Some(x) = queue.pop_front() {
            for idx in 0..26 {
                let y = self.nodes[x].sons[idx];
                if y < usize::MAX {
                    let mut fail = self.nodes[x].fail;
                    while fail > 0 && self.nodes[fail].sons[idx] == usize::MAX {
                        fail = self.nodes[fail].fail;
                    }
                    self.nodes[y].fail = if self.nodes[fail].sons[idx] < usize::MAX {
                        self.nodes[fail].sons[idx]
                    } else {
                        0
                    };

                    queue.push_back(y);
                }
            }
        }
    }

    // fn search(&self, acc: i32, word: &str, dp: &mut [i32]) {
    //     let mut cur = 0;
    //     let mut dp_iter = dp.iter_mut();
    //     for &c in word.as_bytes() {
    //         let idx = (c - b'a') as usize;
    //         if self.nodes[cur].sons[idx] < usize::MAX {
    //             cur = self.nodes[cur].sons[idx];
    //             if let Some(f) = dp_iter.next() {
    //                 if *f > acc + 1 {
    //                     *f = acc + 1;
    //                 }
    //             } else {
    //                 return;
    //             }
    //         } else {
    //             return;
    //         }
    //     }
    // }

    fn solve(&self, s: &str) -> i32 {
        let mut dp = vec![i32::MAX; s.len()];

        let mut cur = 0;
        for (i, &c) in s.as_bytes().iter().enumerate() {
            let idx = (c - b'a') as usize;

            if self.nodes[cur].sons[idx] < usize::MAX {
                cur = self.nodes[cur].sons[idx];
                dp[i] = if i > 0 { dp[i - 1] } else { 1 };
            } else {
                let mut fail = self.nodes[cur].fail;
                while fail > 0 && self.nodes[fail].sons[idx] == usize::MAX {
                    fail = self.nodes[fail].fail;
                }

                if self.nodes[fail].sons[idx] < usize::MAX {
                    cur = self.nodes[fail].sons[idx];
                    dp[i] = 1 + dp[i - self.nodes[cur].dep];
                } else {
                    return -1;
                }
            }
        }

        dp[s.len() - 1]
    }

    #[cfg(debug_assertions)]
    fn show(&self) {
        for (i, node) in self.nodes.iter().enumerate() {
            println!("Node {}:  fail: {} is_end: {}", i, node.fail, node.is_end);
            println!("Sons:");
            for (j, &son) in node.sons.iter().enumerate() {
                if son < usize::MAX {
                    println!("{} -> {}", (b'a' + j as u8) as char, son);
                }
            }
        }
    }
}

impl Solution {
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        let mut trie = Trie::new();
        for word in words {
            trie.insert(&word);
        }
        trie.build();

        #[cfg(debug_assertions)]
        trie.show();

        trie.solve(&target)

        // let mut dp = vec![i32::MAX; target.len()];
        // trie.search(0, &target, &mut dp);

        // for i in 0..dp.len() - 1 {
        //     if dp[i] < i32::MAX {
        //         trie.search(dp[i], &target[i + 1..], &mut dp[i + 1..]);
        //     }
        // }

        // match dp.last() {
        //     Some(&x) if x < i32::MAX => x,
        //     _ => -1,
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec_owned;

    #[test]
    fn sample1() {
        let words = nested_vec_owned!["abc", "aaaaa", "bcdef"];
        let target = "aabcdabc".to_string();
        assert_eq!(Solution::min_valid_strings(words, target), 3);
    }

    #[test]
    fn sample2() {
        let words = nested_vec_owned!["abababab", "ab"];
        let target = "ababaababa".to_string();
        assert_eq!(Solution::min_valid_strings(words, target), 2);
    }

    #[test]
    fn sample3() {
        let words = nested_vec_owned!["abcdef"];
        let target = "xyz".to_string();
        assert_eq!(Solution::min_valid_strings(words, target), -1);
    }
}
