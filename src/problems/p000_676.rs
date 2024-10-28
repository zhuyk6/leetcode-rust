use std::collections::HashMap;

struct Node {
    sons: HashMap<char, Box<Node>>,
    is_end: bool,
}

impl Node {
    fn new() -> Self {
        Node {
            sons: HashMap::new(),
            is_end: false,
        }
    }
}

pub struct MagicDictionary {
    trie: Box<Node>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl MagicDictionary {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        MagicDictionary {
            trie: Box::new(Node::new()),
        }
    }

    pub fn insert(&mut self, s: String) {
        let mut t = &mut self.trie;
        for c in s.chars() {
            t.sons.entry(c).or_insert_with(|| Box::new(Node::new()));
            t = t.sons.get_mut(&c).unwrap();
        }
        t.is_end = true;
    }

    pub fn build_dict(&mut self, dictionary: Vec<String>) {
        dictionary.into_iter().for_each(|s| self.insert(s));
    }

    fn dfs<A>(t: &Node, mut cs: A, change: bool) -> bool
    where
        A: Iterator<Item = char> + Clone,
    {
        if let Some(c) = cs.next() {
            match t.sons.contains_key(&c) {
                true => {
                    Self::dfs(t.sons.get(&c).unwrap(), cs.clone(), change)
                        || match change {
                            true => false,
                            false => t
                                .sons
                                .iter()
                                .filter(|(&k, _)| k != c)
                                .any(|(_, v)| Self::dfs(v, cs.clone(), true)),
                        }
                }
                false => match change {
                    true => false,
                    false => t.sons.iter().any(|(_, v)| Self::dfs(v, cs.clone(), true)),
                },
            }
        } else {
            t.is_end && change
        }
    }

    pub fn search(&self, search_word: String) -> bool {
        Self::dfs(&self.trie, search_word.chars(), false)
    }
}

/**
 * Your MagicDictionary object will be instantiated and called as such:
 * let obj = MagicDictionary::new();
 * obj.build_dict(dictionary);
 * let ret_2: bool = obj.search(searchWord);
 */
#[cfg(test)]
mod tests {
    use super::MagicDictionary;

    #[test]
    fn example() {
        let mut dict = MagicDictionary::new();
        let dics = ["hello", "leetcode"];
        let dics: Vec<String> = dics.into_iter().map(|s| s.to_string()).collect();

        dict.build_dict(dics);

        assert!(!dict.search("hello".to_string()));
        assert!(dict.search("hhllo".to_string()));
        assert!(!dict.search("hell".to_string()));
        assert!(!dict.search("leetcoded".to_string()));
    }

    #[test]
    fn test1() {
        let mut dict = MagicDictionary::new();
        let words = vec![
            "hello".to_string(),
            "hallo".to_string(),
            "leetcode".to_string(),
        ];
        dict.build_dict(words);

        assert!(dict.search("hello".to_string()));
    }
}
