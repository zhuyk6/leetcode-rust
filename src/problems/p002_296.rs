use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node {
    val: char,
    next: Option<Rc<RefCell<Node>>>,
    prev: Weak<RefCell<Node>>,
}

pub struct TextEditor {
    head: Rc<RefCell<Node>>,
    cur: Rc<RefCell<Node>>,
}

impl TextEditor {
    pub fn new() -> Self {
        let head = Rc::new(RefCell::new(Node {
            val: '^',
            next: None,
            prev: Weak::new(),
        }));
        Self {
            head: head.clone(),
            cur: head.clone(),
        }
    }

    pub fn add_text(&mut self, text: String) {
        let mut x = self.cur.clone();
        let end_ptr = x.borrow_mut().next.take();
        for c in text.chars() {
            let new_node = Rc::new(RefCell::new(Node {
                val: c,
                next: None,
                prev: Rc::downgrade(&x),
            }));
            x.borrow_mut().next = Some(new_node.clone());
            x = new_node;
        }
        x.borrow_mut().next = end_ptr.clone();
        if let Some(next) = end_ptr {
            next.borrow_mut().prev = Rc::downgrade(&x);
        }
        self.cur = x;
    }

    pub fn delete_text(&mut self, mut k: i32) -> i32 {
        let mut num_del = 0;
        let mut x = self.cur.clone();
        while k > 0 && !Rc::ptr_eq(&self.head, &x) {
            num_del += 1;
            k -= 1;
            let pre = x.borrow().prev.upgrade().unwrap();
            pre.borrow_mut().next = x.borrow().next.clone();
            if let Some(y) = x.borrow_mut().next.take() {
                y.borrow_mut().prev = Rc::downgrade(&pre);
            }
            x = pre;
        }
        self.cur = x;

        num_del
    }

    fn get_prev_text(&self) -> String {
        let mut ans: Vec<char> = Vec::new();
        let mut x = self.cur.clone();
        let mut left = 10;
        while left > 0 && !Rc::ptr_eq(&self.head, &x) {
            left -= 1;
            ans.push(x.borrow().val);
            let pre = x.borrow().prev.upgrade().unwrap();
            x = pre;
        }
        ans.into_iter().rev().collect()
    }

    pub fn cursor_left(&mut self, mut k: i32) -> String {
        let mut x = self.cur.clone();
        while k > 0 && !Rc::ptr_eq(&self.head, &x) {
            k -= 1;
            let pre = x.borrow().prev.upgrade().unwrap();
            x = pre;
        }
        self.cur = x;
        self.get_prev_text()
    }

    pub fn cursor_right(&mut self, mut k: i32) -> String {
        let mut x = self.cur.clone();
        while k > 0 && x.borrow().next.is_some() {
            k -= 1;
            let next = x.borrow().next.as_ref().unwrap().clone();
            x = next;
        }
        self.cur = x;
        self.get_prev_text()
    }

    pub fn show(&self) {
        println!("Show:");
        let mut x = self.head.clone();
        while let Some(next) = x.clone().borrow().next.clone() {
            print!("{}", next.borrow().val);
            x = next;
        }
        println!();

        let mut x = self.head.clone();
        while let Some(next) = x.clone().borrow().next.clone() {
            print!(
                "{}",
                if Rc::ptr_eq(&self.cur, &next) {
                    '|'
                } else {
                    ' '
                }
            );
            x = next;
        }
        println!();
    }
}

impl Default for TextEditor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let mut editor = TextEditor::new();
        editor.show();
        editor.add_text("leetcode".to_string());
        editor.show();
        assert_eq!(editor.delete_text(4), 4);
        editor.show();
        editor.add_text("practice".to_string());
        editor.show();
        assert_eq!(editor.cursor_right(3), "etpractice".to_string());
        editor.show();
        assert_eq!(editor.cursor_left(8), "leet".to_string());
        editor.show();
        assert_eq!(editor.delete_text(10), 4);
        editor.show();
        assert_eq!(editor.cursor_left(2), "".to_string());
        editor.show();
        assert_eq!(editor.cursor_right(6), "practi".to_string());
        editor.show();
    }
}
