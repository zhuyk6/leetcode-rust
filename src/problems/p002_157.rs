use std::collections::HashMap;

#[allow(dead_code)]
struct DFS<'a> {
    vis: Vec<bool>,
    to: &'a Vec<Vec<usize>>,
    cnt: &'a HashMap<usize, i32>
}

impl <'a> DFS<'a> {
    #[allow(dead_code)]
    pub fn new(n: usize, to: &'a Vec<Vec<usize>>, cnt: &'a HashMap<usize, i32>) -> Self {
        DFS { vis: vec![false; n], to , cnt}
    }

    #[allow(dead_code)]
    pub fn dfs(&mut self, x: usize) -> i32 {
        if self.vis[x] {
            return 0;
        }
        self.vis[x] = true;
        let mut ans = self.cnt[&x];

        for &y in &self.to[x] {
            if !self.vis[y] {
                ans += self.dfs(y);
            }
        }

        ans
    }
}

#[allow(dead_code)]
pub fn group_strings(words: Vec<String>) -> Vec<i32> {
    const M: usize = 26;
    
    let codes = words.into_iter()
        .map(|s| {
            let mut x = 0;
            for c in s.bytes() {
                x += 1 << (c - b'a') as usize;
            }
            x
        }).collect::<Vec<usize>>();
    
    println!("codes: {:?}", codes);

    let mut map: HashMap<usize, (usize, i32)> = HashMap::new();
    for (i, &c) in codes.iter().enumerate() {
        map.entry(c).or_insert((i, 0)).1 += 1;
    }

    fn links(x: usize, map: &HashMap<usize, (usize, i32)>) -> Vec<usize> {
        let mut v = vec![];
        // delete or add
        for b in 0..M {
            let y = x ^ (1 << b);
            if map.contains_key(&y) {
                v.push(map[&y].0);
            }
        }
        // modify
        for b1 in 0..M {
            // b1 should be 1
            if x & (1 << b1) == 0 { continue; }
            for b2 in 0..M {
                // b2 should be 0
                if x & (1 << b2) > 0 {continue;}
                let y = x ^ (1 << b1) | (1 << b2);
                if map.contains_key(&y) {
                    v.push(map[&y].0);
                }
            }
        }
        v
    }

    let n = codes.len();
    let mut to = vec![vec![]; n];
    for (&code, &(idx, _num)) in &map {
        for y in links(code, &map) {
            to[idx].push(y);
        }
    }

    println!("map: {:?}", map);
    println!("to: {:#?}", to);

    let mut ans = vec![0, 0];
    let cnt: HashMap<usize, i32> = map.values()
        .cloned()
        .collect();

    let mut dfs = DFS::new(n, &to, &cnt);

    for (&_code, &(idx, _num)) in &map {
        let tmp = dfs.dfs(idx);
        if tmp > 0 {
            ans[0] += 1;
            ans[1] = ans[1].max(tmp);
        }
    }

    ans
}

#[test]
fn example() {
    let words = ["a","b","ab","cde"];
    let words = words.into_iter().map(|s| s.to_string()).collect();
    assert_eq!(group_strings(words), vec![2, 3]);
}
