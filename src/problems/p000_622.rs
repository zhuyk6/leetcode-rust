pub struct MyCircularQueue {
    que: Vec<i32>,
    head: usize,
    tail: usize,
    full: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl MyCircularQueue {
    pub fn new(k: i32) -> Self {
        MyCircularQueue {
            que: vec![0; k as usize],
            head: 0,
            tail: 0,
            full: false,
        }
    }

    pub fn en_queue(&mut self, value: i32) -> bool {
        if self.full {
            false
        } else {
            self.que[self.tail] = value;
            self.tail = (self.tail + 1) % self.que.len();
            if self.tail == self.head {
                self.full = true;
            }
            true
        }
    }

    pub fn de_queue(&mut self) -> bool {
        if self.tail == self.head && !self.full {
            false
        } else {
            self.head = (self.head + 1) % self.que.len();
            self.full = false;
            true
        }
    }

    pub fn front(&self) -> i32 {
        if self.tail == self.head && !self.full {
            -1
        } else {
            self.que[self.head]
        }
    }

    pub fn rear(&self) -> i32 {
        if self.tail == self.head && !self.full {
            -1
        } else {
            self.que[(self.tail + self.que.len() - 1) % self.que.len()]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tail == self.head && !self.full
    }

    pub fn is_full(&self) -> bool {
        self.full
    }
}

#[test]
fn example() {
    let mut circular_queue = MyCircularQueue::new(3); // 设置长度为 3
    circular_queue.en_queue(1); // 返回 true
    circular_queue.en_queue(2); // 返回 true
    circular_queue.en_queue(3); // 返回 true
    circular_queue.en_queue(4); // 返回 false，队列已满
    circular_queue.rear(); // 返回 3
    circular_queue.is_full(); // 返回 true
    circular_queue.de_queue(); // 返回 true

    println!("que: {:?}", circular_queue.que);
    println!(
        "head: {}, tail: {}",
        circular_queue.head, circular_queue.tail
    );

    assert!(circular_queue.en_queue(4));
    circular_queue.rear(); // 返回 4
}
