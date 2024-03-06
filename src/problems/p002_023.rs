#[derive(Default)]
struct Node {
    son: [Option<Box<Node>>; 10],
    end_of_words: Vec<usize>,
}

impl Node {
    pub fn new() -> Self {
        Node::default()
    }

    pub fn insert<I>(&mut self, word: I, idx: usize)
    where
        I: Iterator<Item = u8>,
    {
        let mut node = self;
        for c in word {
            let c = (c - b'0') as usize;
            node = node.son[c].get_or_insert_with(|| Box::new(Node::new()));
        }
        node.end_of_words.push(idx);
    }

    pub fn search<I>(&self, iter: I, v: &mut [usize])
    where
        I: Iterator<Item = (usize, u8)>,
    {
        let mut node = self;
        for (i, c) in iter {
            let c = (c - b'0') as usize;
            if let Some(next) = node.son[c].as_deref() {
                node = next;
                for &j in &node.end_of_words {
                    v[j] = i;
                }
            } else {
                break;
            }
        }
    }
}

#[allow(unused, clippy::needless_range_loop)]
pub fn num_of_pairs(nums: Vec<String>, target: String) -> i32 {
    let n = nums.len();

    let mut root = Node::new();
    for (i, s) in nums.iter().enumerate() {
        root.insert(s.bytes(), i);
    }

    let mut st = vec![usize::MAX; n];
    root.search(target.bytes().enumerate(), &mut st);

    let mut root = Node::new();
    for (i, s) in nums.iter().enumerate() {
        root.insert(s.bytes().rev(), i);
    }
    let mut ed = vec![usize::MAX; n];
    root.search(target.bytes().enumerate().rev(), &mut ed);

    println!("st: {:?}", st);
    println!("ed: {:?}", ed);

    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            if i == j || st[i] == usize::MAX || ed[j] == usize::MAX {
                continue;
            }
            if st[i] + 1 == ed[j] {
                println!("(i, j) = ({i}, {j})");
                ans += 1;
            }
        }
    }
    ans
}

#[test]
fn example1() {
    let nums = vec![
        "777".to_string(),
        "7".to_string(),
        "77".to_string(),
        "77".to_string(),
    ];
    let target = "7777".to_string();
    assert_eq!(num_of_pairs(nums, target), 4);
}

#[test]
fn example2() {
    let nums = vec![
        "123".to_string(),
        "4".to_string(),
        "12".to_string(),
        "34".to_string(),
    ];
    let target = "1234".to_string();
    assert_eq!(num_of_pairs(nums, target), 2);
}

#[test]
fn wrong() {
    let nums = vec!["1".to_string(), "111".to_string()];
    let target = "11".to_string();
    assert_eq!(num_of_pairs(nums, target), 0);
}
