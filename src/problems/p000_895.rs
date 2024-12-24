use std::collections::HashMap;

pub struct FreqStack {
    concurrence: HashMap<i32, usize>,
    stacks: Vec<Vec<i32>>,
}

impl FreqStack {
    pub fn new() -> Self {
        FreqStack {
            concurrence: HashMap::new(),
            stacks: vec![vec![]],
        }
    }

    pub fn push(&mut self, val: i32) {
        let freq = {
            let tmp = self.concurrence.entry(val).or_default();
            *tmp += 1;
            *tmp
        };
        if freq >= self.stacks.len() {
            self.stacks.push(vec![]);
        }
        self.stacks[freq].push(val);
    }

    pub fn pop(&mut self) -> i32 {
        let ret = self.stacks.last_mut().unwrap().pop().unwrap();
        self.concurrence.entry(ret).and_modify(|e| *e -= 1);
        if self.stacks.len() > 1 && self.stacks.last().unwrap().is_empty() {
            self.stacks.pop();
        }
        ret
    }
}

impl Default for FreqStack {
    fn default() -> Self {
        Self::new()
    }
}
