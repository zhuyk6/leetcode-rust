struct MyQueue {
    head: Vec<i32>,
    tail: Vec<i32>,
}

#[allow(dead_code)]
impl MyQueue {
    fn new() -> Self {
        MyQueue {
            head: Vec::new(),
            tail: Vec::new(),
        }
    }

    fn adjust(&mut self) {
        assert!(self.head.is_empty());
        self.head = std::mem::take(&mut self.tail).into_iter().rev().collect();
    }

    fn push(&mut self, x: i32) {
        self.tail.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.head.is_empty() {
            self.adjust();
        }
        self.head.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if self.head.is_empty() {
            self.adjust();
        }
        *self.head.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.head.is_empty() && self.tail.is_empty()
    }
}

#[test]
fn test1() {
    let mut que = MyQueue::new();
    que.push(1);
    que.push(2);
    assert_eq!(que.peek(), 1);
    assert_eq!(que.pop(), 1);
    assert!(!que.empty());
}
