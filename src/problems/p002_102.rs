use std::cmp::Reverse;
use std::collections::BinaryHeap;

type Pair = (i32, Reverse<String>);

#[derive(Debug, Clone, Default)]
pub struct SORTracker {
    heap1: BinaryHeap<Reverse<Pair>>,
    heap2: BinaryHeap<Pair>,
    qid: usize,
}

impl SORTracker {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, name: String, score: i32) {
        match self.heap1.len().cmp(&(self.qid + 1)) {
            std::cmp::Ordering::Less => self.heap1.push(Reverse((score, Reverse(name)))),
            std::cmp::Ordering::Equal => {
                let Reverse((s, Reverse(n))) = self.heap1.pop().unwrap();
                if score > s || (score == s && name < n) {
                    self.heap1.push(Reverse((score, Reverse(name))));
                    self.heap2.push((s, Reverse(n)));
                } else {
                    self.heap1.push(Reverse((s, Reverse(n))));
                    self.heap2.push((score, Reverse(name)));
                }
            }
            std::cmp::Ordering::Greater => panic!("Error!"),
        }

        // dbg!(&self.heap1);
        // dbg!(&self.heap2);
    }

    pub fn get(&mut self) -> String {
        self.qid += 1;
        assert_eq!(self.qid, self.heap1.len());
        let Reverse(p) = self.heap1.peek().unwrap();
        let Reverse(ret) = p.1.to_owned();

        if let Some((s, n)) = self.heap2.pop() {
            self.heap1.push(Reverse((s, n)));
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let mut sol = SORTracker::new();
        sol.add("bradford".to_string(), 2);
        sol.add("branford".into(), 3);

        assert_eq!(sol.get().as_str(), "branford");

        sol.add("alps".into(), 2);

        assert_eq!(sol.get().as_str(), "alps");
    }
}
