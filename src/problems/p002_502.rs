use std::collections::{BTreeSet, HashMap};

#[derive(Debug)]
struct Node {
    start: u32,
    len: u32,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct Allocator {
    head: Node,
    mem: HashMap<i32, BTreeSet<(u32, u32)>>,
}

impl Allocator {
    pub fn new(n: i32) -> Self {
        let node = Node {
            start: 0,
            len: n as u32,
            next: None,
        };
        Self {
            head: Node {
                start: 0,
                len: 0,
                next: Some(Box::new(node)),
            },
            mem: HashMap::new(),
        }
    }

    pub fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        let size = size as u32;

        let mut x = &mut self.head;
        while let Some(mut y) = x.next.take() {
            match y.len.cmp(&size) {
                std::cmp::Ordering::Less => {
                    x.next = Some(y);
                    x = x.next.as_mut().unwrap();
                }
                std::cmp::Ordering::Equal => {
                    x.next = y.next.take();
                    self.mem.entry(m_id).or_default().insert((y.start, size));
                    return y.start as i32;
                }
                std::cmp::Ordering::Greater => {
                    let z = Box::new(Node {
                        start: y.start + size,
                        len: y.len - size,
                        next: y.next.take(),
                    });
                    x.next = Some(z);
                    self.mem.entry(m_id).or_default().insert((y.start, size));
                    return y.start as i32;
                }
            }
        }

        -1
    }

    fn insert(&mut self, start: u32, len: u32) {
        let mut x = &mut self.head;
        while let Some(mut y) = x.next.take() {
            if y.start > start {
                // merge y and z if possible
                let z = if start + len < y.start {
                    Box::new(Node {
                        start,
                        len,
                        next: Some(y),
                    })
                } else {
                    y.start = start;
                    y.len += len;
                    y
                };

                // merge x and z if possible
                if x.len == 0 || x.start + x.len < z.start {
                    x.next = Some(z);
                } else {
                    x.len += z.len;
                    x.next = z.next;
                }
                return;
            } else {
                x.next = Some(y);
                x = x.next.as_mut().unwrap();
            }
        }
        x.next = Some(Box::new(Node {
            start,
            len,
            next: None,
        }));
    }

    pub fn free_memory(&mut self, m_id: i32) -> i32 {
        if let Some(blocks) = self.mem.remove(&m_id) {
            let mut ans = 0;

            blocks.into_iter().for_each(|(start, len)| {
                ans += len;
                self.insert(start, len);
            });

            ans as i32
        } else {
            0
        }
    }

    pub fn show(&self) {
        println!("Link: ");
        let mut x = &self.head;
        while let Some(y) = x.next.as_ref() {
            println!("{} {}", y.start, y.len);
            x = y;
        }

        println!("Mem: {:#?}", self.mem);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let mut allocator = Allocator::new(10);

        assert_eq!(allocator.allocate(1, 1), 0);
        allocator.show();

        assert_eq!(allocator.allocate(1, 2), 1);
        allocator.show();

        assert_eq!(allocator.allocate(1, 3), 2);
        allocator.show();

        assert_eq!(allocator.free_memory(2), 1);
        allocator.show();

        assert_eq!(allocator.allocate(3, 4), 3);
        allocator.show();

        assert_eq!(allocator.allocate(1, 1), 1);
        allocator.show();

        assert_eq!(allocator.allocate(1, 1), 6);
        allocator.show();

        assert_eq!(allocator.free_memory(1), 3);
        allocator.show();

        assert_eq!(allocator.allocate(10, 2), -1);
        allocator.show();

        assert_eq!(allocator.free_memory(7), 0);
        allocator.show();
    }

    #[test]
    fn issue() {
        let mut allocator = Allocator::new(7);

        assert_eq!(allocator.allocate(3, 1), 0);
        allocator.show();

        assert_eq!(allocator.allocate(5, 2), -1);
        allocator.show();

        assert_eq!(allocator.free_memory(1), 3);
        allocator.show();

        assert_eq!(allocator.free_memory(3), 0);
        allocator.show();
    }
}
