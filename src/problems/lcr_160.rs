use std::cmp::Reverse;
use std::collections::BinaryHeap;

// make sure h2.len <= h1.len <= h2.len + 1
#[derive(Default)]
struct MedianFinder {
    h1: BinaryHeap<i32>,
    h2: BinaryHeap<Reverse<i32>>,
}

#[allow(unused, clippy::collapsible_else_if)]
impl MedianFinder {
    fn new() -> Self {
        Default::default()
    }

    #[inline]
    fn is_odd(&self) -> bool {
        (self.h1.len() + self.h2.len()) % 2 == 1
    }

    fn add_num(&mut self, num: i32) {
        if self.is_odd() {
            if let Some(v) = self.h1.pop() {
                if num <= v {
                    self.h1.push(num);
                    self.h2.push(Reverse(v));
                } else {
                    self.h1.push(v);
                    self.h2.push(Reverse(num));
                }
            }
        } else {
            if let Some(Reverse(v)) = self.h2.pop() {
                if num <= v {
                    self.h1.push(num);
                    self.h2.push(Reverse(v));
                } else {
                    self.h1.push(v);
                    self.h2.push(Reverse(num));
                }
            } else {
                self.h1.push(num);
            }
        }
        assert!(self.h1.len() >= self.h2.len() && self.h1.len() <= self.h2.len() + 1);
    }

    fn find_median(&self) -> f64 {
        if self.is_odd() {
            let v = self.h1.peek().unwrap();
            (*v) as f64
        } else {
            let v1 = self.h1.peek().unwrap();
            let Reverse(v2) = self.h2.peek().unwrap();
            ((*v1) as f64 + (*v2) as f64) / 2.0
        }
    }
}
