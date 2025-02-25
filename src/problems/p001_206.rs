use std::{cell::RefCell, rc::Rc};

use rand;

const MAX_LEVEL: usize = 16;
const PROB: f64 = 0.5;

fn rand_level() -> usize {
    let mut level = 1;
    while rand::random::<f64>() < PROB && level < MAX_LEVEL {
        level += 1;
    }
    level
}

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Clone)]
struct Node {
    val: i32,
    next_nodes: Vec<Link>,
}

impl Node {
    fn new(val: i32, level: usize) -> Self {
        Self {
            val,
            next_nodes: vec![None; level],
        }
    }

    #[inline]
    fn level(&self) -> usize {
        self.next_nodes.len()
    }
}

#[derive(Debug, Clone)]
pub struct Skiplist {
    head: Rc<RefCell<Node>>,
}

impl Skiplist {
    pub fn new() -> Self {
        Self {
            head: Rc::new(RefCell::new(Node::new(i32::MIN, MAX_LEVEL))),
        }
    }

    fn find_gre_node(&self, val: i32, pre: &mut [Link]) -> Link {
        let mut x = self.head.clone();
        let mut l = MAX_LEVEL - 1;
        loop {
            let y = x.borrow().next_nodes[l].clone();
            if y.is_none() || y.as_ref().unwrap().borrow().val >= val {
                pre[l] = Some(x.clone()); // record predecessor
                if l > 0 {
                    l -= 1;
                } else {
                    return y;
                }
            } else {
                x = y.unwrap();
            }
        }
    }

    pub fn search(&self, target: i32) -> bool {
        let mut pre = [const { None }; MAX_LEVEL];
        let x = self.find_gre_node(target, &mut pre);
        // x.is_some_and(|node| node.borrow().val == target)
        x.is_some() && x.unwrap().borrow().val == target
    }

    pub fn add(&self, num: i32) {
        let mut pre = [const { None }; MAX_LEVEL];
        self.find_gre_node(num, &mut pre);
        let new_node = Rc::new(RefCell::new(Node::new(num, rand_level())));
        let level = new_node.borrow().level();

        #[allow(clippy::needless_range_loop)]
        for l in 0..level {
            let last_node = pre[l].clone().unwrap();
            new_node.borrow_mut().next_nodes[l] = last_node.borrow_mut().next_nodes[l].take();
            last_node.borrow_mut().next_nodes[l] = Some(new_node.clone());
        }
    }

    pub fn erase(&self, num: i32) -> bool {
        let mut pre = [const { None }; MAX_LEVEL];
        let x = self.find_gre_node(num, &mut pre);
        // if x.as_ref().is_none_or(|node| node.borrow().val != num) {
        //     return false;
        // }
        if x.is_none() || x.as_ref().unwrap().borrow().val != num {
            return false;
        }
        let node = x.unwrap();
        let level = node.borrow().level();

        #[allow(clippy::needless_range_loop)]
        for l in 0..level {
            let last_node = pre[l].clone().unwrap();
            last_node.borrow_mut().next_nodes[l] = node.borrow_mut().next_nodes[l].take();
        }

        true
    }
}

impl Default for Skiplist {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let obj = Skiplist::new();
        obj.add(1);
        obj.add(2);
        obj.add(3);
        assert!(!obj.search(0));
        obj.add(4);
        assert!(obj.search(1));
        assert!(!obj.erase(0));
        assert!(obj.erase(1));
        assert!(!obj.search(1));
    }
}
