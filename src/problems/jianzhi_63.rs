#[derive(Default)]
struct Node {
    son: [Option<Box<Node>>; 26],
    end_of_word: Option<String>,
}

#[derive(Default)]
struct Trie {
    root: Node,
}

impl Trie {
    pub fn insert(&mut self, s: String) {
        let mut cur = &mut self.root;
        for c in s.bytes() {
            let c = (c - b'a') as usize;
            cur = cur.son[c].get_or_insert_with(Box::<Node>::default);
        }
        cur.end_of_word = Some(s);
    }

    pub fn find(&self, w: &str) -> Option<String> {
        let mut cur = &self.root;
        for c in w.bytes() {
            let c = (c - b'a') as usize;
            match cur.son[c].as_deref() {
                None => return None,
                Some(node) => cur = node,
            }
            if cur.end_of_word.is_some() {
                return cur.end_of_word.clone();
            }
        }
        None
    }
}

pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
    let mut trie = Trie::default();
    for s in dictionary {
        trie.insert(s);
    }
    let words = sentence.split(' ');
    words
        .map(|w| match trie.find(w) {
            Some(s) => s,
            None => w.to_string(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn example() {
    let dictionary = vec!["cat".to_string(), "bat".to_string(), "rat".to_string()];
    let sentence = "the cattle was rattled by the battery".to_string();
    assert_eq!(
        replace_words(dictionary, sentence),
        "the cat was rat by the bat".to_string()
    );
}
