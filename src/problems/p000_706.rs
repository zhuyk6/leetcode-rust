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

struct MyHashMap {
    cnt: usize,
    cap: usize,
    arr: Vec<Vec<(Key, Val)>>,
}

const MIN_CAP: usize = 8;
const ALPHA: usize = 2;
const BETA: usize = 2;

#[allow(unused)]
impl MyHashMap {
    fn new() -> Self {
        MyHashMap {
            cnt: 0,
            cap: MIN_CAP,
            arr: vec![vec![]; MIN_CAP],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
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

    fn expand(&mut self) {
        use std::mem;
        let mut arr = mem::take(&mut self.arr);
        self.cap *= BETA;
        self.arr = vec![vec![]; self.cap];
        self.cnt = 0;
        arr.into_iter()
            .for_each(|v| v.into_iter().for_each(|(k, v)| self.put(k, v)));
    }

    fn get(&self, key: i32) -> i32 {
        let h = key.hash();
        let p = h % self.cap;
        self.arr[p]
            .iter()
            .find(|(k, v)| *k == key)
            .map(|(k, v)| *v)
            .unwrap_or(-1)
    }

    fn remove(&mut self, key: i32) {
        let h = key.hash();
        let p = h % self.cap;
        if let Some(pos) = self.arr[p].iter().position(|(k, _)| *k == key) {
            self.arr[p].remove(pos);
            self.cnt -= 1;
        }
    }
}
