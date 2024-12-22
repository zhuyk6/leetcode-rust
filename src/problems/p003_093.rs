pub struct Solution;

const MAX_CHAR: usize = 26;

#[derive(Debug, Clone)]
struct Index {
    len: usize,
    index: usize,
}

impl Default for Index {
    fn default() -> Self {
        Index {
            len: usize::MAX,
            index: usize::MAX,
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    sons: [Option<Box<Node>>; MAX_CHAR],
    shortest_index: Index,
}

impl Node {
    fn new(index: Index) -> Self {
        Node {
            sons: [const { None }; MAX_CHAR],
            shortest_index: index,
        }
    }
}

struct Trie {
    root: Box<Node>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: Box::new(Node::new(Index::default())),
        }
    }

    fn insert(&mut self, word: &str, index: usize) {
        let index = Index {
            len: word.len(),
            index,
        };
        let mut node = &mut self.root;
        if node.shortest_index.len > index.len {
            node.shortest_index = index.clone();
        }
        for &c in word.as_bytes().iter().rev() {
            let c = (c - b'a') as usize;
            node = node.sons[c].get_or_insert_with(|| Box::new(Node::new(Index::default())));
            if node.shortest_index.len > index.len {
                node.shortest_index = index.clone();
            }
        }
    }

    fn query(&self, word: &str) -> Index {
        let mut node = &self.root;
        for &c in word.as_bytes().iter().rev() {
            let c = (c - b'a') as usize;
            if let Some(son) = &node.sons[c] {
                node = son;
            } else {
                return node.shortest_index.clone();
            }
        }
        node.shortest_index.clone()
    }
}

impl Solution {
    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        let mut trie = Trie::new();
        for (i, word) in words_container.into_iter().enumerate() {
            trie.insert(&word, i);
        }

        words_query
            .into_iter()
            .map(|word| trie.query(&word).index as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nested_vec_owned;

    #[test]
    fn sample1() {
        let words_container = nested_vec_owned!["abcd", "bcd", "xbcd"];
        let words_query = nested_vec_owned!["cd", "bcd", "xyz"];
        assert_eq!(
            Solution::string_indices(words_container, words_query),
            vec![1, 1, 1]
        )
    }

    #[test]
    fn sample2() {
        let words_container = nested_vec_owned!["abcdefgh", "poiuygh", "ghghgh"];
        let words_query = nested_vec_owned!["gh", "acbfgh", "acbfegh"];
        assert_eq!(
            Solution::string_indices(words_container, words_query),
            vec![2, 0, 2]
        )
    }
}
