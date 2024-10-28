trait Hashable {
    fn hash(&self) -> usize;
}

impl Hashable for i32 {
    fn hash(&self) -> usize {
        *self as usize
    }
}

type Key = i32;
type Val = i32;

pub struct MyHashMap {
    cnt: usize,
    cap: usize,
    arr: Vec<Vec<(Key, Val)>>,
}

const MIN_CAP: usize = 8;
const ALPHA: usize = 2;
const BETA: usize = 2;

impl MyHashMap {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        MyHashMap {
            cnt: 0,
            cap: MIN_CAP,
            arr: vec![vec![]; MIN_CAP],
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let h = key.hash();
        let p = h % self.cap;
        for (k, v) in &mut self.arr[p] {
            if *k == key {
                *v = value;
                return;
            }
        }
        self.arr[p].push((key, value));
        self.cnt += 1;
        if self.cnt > ALPHA * self.cap {
            self.expand();
        }
    }

    pub fn expand(&mut self) {
        use std::mem;
        let arr = mem::take(&mut self.arr);
        self.cap *= BETA;
        self.arr = vec![vec![]; self.cap];
        self.cnt = 0;
        arr.into_iter()
            .for_each(|v| v.into_iter().for_each(|(k, v)| self.put(k, v)));
    }

    pub fn get(&self, key: i32) -> i32 {
        let h = key.hash();
        let p = h % self.cap;
        self.arr[p]
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| *v)
            .unwrap_or(-1)
    }

    pub fn remove(&mut self, key: i32) {
        let h = key.hash();
        let p = h % self.cap;
        if let Some(pos) = self.arr[p].iter().position(|(k, _)| *k == key) {
            self.arr[p].remove(pos);
            self.cnt -= 1;
        }
    }
}
