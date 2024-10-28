use std::{cell::RefCell, collections::HashMap, rc::Rc};

type Key = i32;
type Val = i32;
type Link = Rc<RefCell<Node>>;

struct Node {
    prev: Option<Link>,
    next: Option<Link>,
    item: (Key, Val),
}

impl Node {
    fn new(item: (Key, Val)) -> Self {
        Node {
            prev: None,
            next: None,
            item,
        }
    }
}

pub struct LRUCache {
    capacity: usize,
    head: Link,
    tail: Link,
    key_to_node: HashMap<Key, Link>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let head = Rc::new(RefCell::new(Node::new((-1, -1))));
        let tail = Rc::new(RefCell::new(Node::new((-1, -1))));
        tail.as_ref().borrow_mut().next = Some(head.clone());
        head.as_ref().borrow_mut().prev = Some(tail.clone());

        LRUCache {
            capacity: capacity as usize,
            head,
            tail,
            key_to_node: HashMap::new(),
        }
    }

    pub fn view(&self) {
        println!("Map: {:?}", self.key_to_node.keys().collect::<Vec<_>>());
        print!("Tail -> ");
        let mut cur = self.tail.clone();
        while let Some(next) = cur.clone().as_ref().borrow().next.as_ref() {
            print!("{} -> ", next.as_ref().borrow().item.1);
            cur = next.clone();
        }
        println!();

        print!("Head <- ");
        let mut cur = self.head.clone();
        while let Some(prev) = cur.clone().as_ref().borrow().prev.as_ref() {
            print!("{} <- ", prev.as_ref().borrow().item.1);
            cur = prev.clone();
        }
        println!();
    }

    fn move_to_tail(&self, node: Link) {
        let next = node.as_ref().borrow_mut().next.take().unwrap();
        let last = node.as_ref().borrow_mut().prev.take().unwrap();

        // last -> next
        last.as_ref().borrow_mut().next = Some(next.clone());
        next.as_ref().borrow_mut().prev = Some(last.clone());

        // tail -> node -> tail.next
        let p = self.tail.clone();
        let q = p.as_ref().borrow_mut().next.take().unwrap();
        p.as_ref().borrow_mut().next = Some(node.clone());
        node.as_ref().borrow_mut().next = Some(q.clone());
        q.as_ref().borrow_mut().prev = Some(node.clone());
        node.as_ref().borrow_mut().prev = Some(p.clone());
    }

    pub fn get(&self, key: i32) -> i32 {
        self.key_to_node.get(&key).map_or(-1, |node| {
            self.move_to_tail(node.clone());
            node.as_ref().borrow().item.1
        })
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.key_to_node.get(&key) {
            self.move_to_tail(node.clone());
            node.as_ref().borrow_mut().item.1 = value;
        } else {
            // If full, remove the LRU entry.
            if self.key_to_node.len() >= self.capacity {
                let node = self.head.as_ref().borrow_mut().prev.take().unwrap();
                self.key_to_node
                    .remove(&node.as_ref().borrow().item.0)
                    .unwrap();

                // head <- node.prev
                let p = node.as_ref().borrow_mut().prev.take().unwrap();
                p.as_ref().borrow_mut().next = Some(self.head.clone());
                self.head.as_ref().borrow_mut().prev = Some(p.clone());
            }

            // tail -> node -> tail.next
            let p = self.tail.clone();
            let q = p.as_ref().borrow_mut().next.take().unwrap();
            let node = Rc::new(RefCell::new(Node::new((key, value))));
            p.as_ref().borrow_mut().next = Some(node.clone());
            node.as_ref().borrow_mut().next = Some(q.clone());
            q.as_ref().borrow_mut().prev = Some(node.clone());
            node.as_ref().borrow_mut().prev = Some(p.clone());
            self.key_to_node.insert(key, node);
        }
    }
}

#[test]
fn test1() {
    let mut lru = LRUCache::new(2);
    lru.put(1, 1);
    lru.put(2, 2);
    assert_eq!(lru.get(1), 1);
    lru.put(3, 3);
    assert_eq!(lru.get(2), -1);
    lru.put(4, 4);
    assert_eq!(lru.get(1), -1);
    assert_eq!(lru.get(3), 3);
    assert_eq!(lru.get(4), 4);
}

#[test]
fn test2() {
    let mut lru = LRUCache::new(2);
    lru.view();
    lru.put(1, 0);
    lru.view();
    lru.put(2, 2);
    lru.view();
    assert_eq!(lru.get(1), 0);
    lru.view();
    lru.put(3, 3);
    lru.view();
    assert_eq!(lru.get(2), -1);
    lru.view();
    lru.put(4, 4);
    lru.view();
    assert_eq!(lru.get(1), -1);
    lru.view();
    assert_eq!(lru.get(3), 3);
    lru.view();
    assert_eq!(lru.get(4), 4);
    lru.view();
}
