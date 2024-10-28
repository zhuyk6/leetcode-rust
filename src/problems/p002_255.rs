pub struct Solution;

#[derive(Default)]
struct Node {
    sons: [Option<Box<Node>>; 26],
    cnt: usize,
}

impl Node {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Default)]
struct Trie {
    root: Node,
}

impl Trie {
    fn insert(&mut self, s: &str) {
        let mut x = &mut self.root;
        for c in s.bytes() {
            let idx = (c - b'a') as usize;
            if x.sons[idx].is_none() {
                x.sons[idx] = Some(Box::new(Node::new()));
            }
            x = x.sons[idx].as_mut().unwrap();
        }
        x.cnt += 1;
    }

    fn count_prefix(&self, s: &str) -> usize {
        let mut acc = 0;
        let mut x = &self.root;
        for c in s.bytes() {
            let idx = (c - b'a') as usize;
            if x.sons[idx].is_none() {
                return acc;
            }
            x = x.sons[idx].as_ref().unwrap();
            acc += x.cnt;
        }
        acc
    }
}

impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        let mut trie: Trie = Default::default();
        for s in words.into_iter() {
            trie.insert(s.as_str());
        }
        trie.count_prefix(s.as_str()) as i32
    }
}

#[test]
fn test1() {
    let words = vec![
        "a".into(),
        "b".into(),
        "c".into(),
        "ab".into(),
        "bc".into(),
        "abc".into(),
    ];
    let s = "abc".into();
    assert_eq!(Solution::count_prefixes(words, s), 3);
}
