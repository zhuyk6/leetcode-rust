type Link = Option<Box<Node>>;

struct Node {
    lc: Link,
    rc: Link,
    left: i32,
    right: i32,
    lazy: usize,
    val: usize,
    num: i32,
}

impl Node {
    fn new(left: i32, right: i32) -> Self {
        Node {
            lc: None,
            rc: None,
            left,
            right,
            lazy: 0,
            val: 0,
            num: 0,
        }
    }

    fn insert_left_son(&mut self) {
        let mid = (self.left + self.right) >> 1;
        self.lc = Some(Box::new(Node::new(self.left, mid)));
    }
    fn insert_right_son(&mut self) {
        let mid = (self.left + self.right) >> 1;
        self.rc = Some(Box::new(Node::new(mid + 1, self.right)));
    }

    fn push_down(&mut self) {
        if self.lc.is_none() {
            self.insert_left_son();
        }
        if self.rc.is_none() {
            self.insert_right_son();
        }
        if self.lazy != 0 {
            let lc = self.lc.as_mut().unwrap();
            lc.lazy += self.lazy;
            lc.val += self.lazy;
            lc.num = lc.right - lc.left + 1;

            let rc = self.rc.as_mut().unwrap();
            rc.lazy += self.lazy;
            rc.val += self.lazy;
            rc.num = rc.right - rc.left + 1;

            self.lazy = 0;
        }
    }

    fn update(&mut self) {
        if self.val > 0 {
            self.num = self.right - self.left + 1;
        } else {
            self.num = self.lc.as_ref().unwrap().num + self.rc.as_ref().unwrap().num;
        }
    }

    fn add(&mut self, left: i32, right: i32) {
        if left <= self.left && self.right <= right {
            self.lazy += 1;
            self.val += 1;
            self.num = self.right - self.left + 1;
            return;
        }
        self.push_down();
        // [left, mid] + [mid+1, right]
        let mid = (self.left + self.right) >> 1;
        if left <= mid {
            self.lc.as_mut().unwrap().add(left, right);
        }
        if mid < right {
            self.rc.as_mut().unwrap().add(left, right);
        }
        self.update();
    }

    // fn count(&mut self, left: i32, right: i32) -> i32 {
    //     if left <= self.left && self.right <= right {
    //         return self.num;
    //     }
    //     self.push_down();
    //     let mid = (self.left + self.right) >> 1;
    //     let mut ret = 0;
    //     if left <= mid {
    //         ret += self.lc.as_mut().unwrap().count(left, right);
    //     }
    //     if mid < right {
    //         ret += self.rc.as_mut().unwrap().count(left, right);
    //     }
    //     self.update();
    //     ret
    // }
}

pub struct CountIntervals {
    root: Link,
}

const LEFT: i32 = 1;
const RIGHT: i32 = 1_000_000_000;

impl CountIntervals {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        CountIntervals {
            root: Some(Box::new(Node::new(LEFT, RIGHT))),
        }
    }

    pub fn add(&mut self, left: i32, right: i32) {
        self.root.as_mut().unwrap().add(left, right);
    }

    pub fn count(&self) -> i32 {
        self.root.as_ref().unwrap().num
    }
}

#[test]
fn test1() {
    let mut cnt = CountIntervals::new();
    cnt.add(2, 3);
    cnt.add(7, 10);
    assert_eq!(cnt.count(), 6);
    cnt.add(5, 8);
    assert_eq!(cnt.count(), 8);
}
