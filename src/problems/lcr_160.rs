use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug)]
enum State {
    Balanced {
        h1: BinaryHeap<i32>,
        h2: BinaryHeap<Reverse<i32>>,
    },
    Unbalanced {
        h1: BinaryHeap<i32>,
        mid: i32,
        h2: BinaryHeap<Reverse<i32>>,
    },
}

impl State {
    fn add_num(self, num: i32) -> Self {
        match self {
            State::Balanced { mut h1, mut h2 } => {
                if h1.is_empty() {
                    State::Unbalanced { h1, mid: num, h2 }
                } else {
                    let num1 = h1.pop().unwrap();
                    let num2 = h2.pop().unwrap().0;
                    let mut nums = [num1, num2, num];
                    nums.sort_unstable();
                    h1.push(nums[0]);
                    h2.push(Reverse(nums[2]));
                    State::Unbalanced {
                        h1,
                        mid: nums[1],
                        h2,
                    }
                }
            }
            State::Unbalanced {
                mut h1,
                mid,
                mut h2,
            } => {
                match num.cmp(&mid) {
                    std::cmp::Ordering::Less => {
                        h1.push(num);
                        h2.push(Reverse(mid));
                    }
                    _ => {
                        h1.push(mid);
                        h2.push(Reverse(num));
                    }
                }
                State::Balanced { h1, h2 }
            }
        }
    }

    fn find_median(&self) -> f64 {
        match self {
            State::Balanced { h1, h2 } => {
                let num1 = *h1.peek().unwrap();
                let num2 = h2.peek().unwrap().0;
                (num1 + num2) as f64 / 2.0
            }
            State::Unbalanced { mid, .. } => (*mid) as f64,
        }
    }
}

pub struct MedianFinder {
    state: Option<State>,
}

impl MedianFinder {
    pub fn new() -> Self {
        MedianFinder {
            state: Some(State::Balanced {
                h1: BinaryHeap::new(),
                h2: BinaryHeap::new(),
            }),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        self.state = self.state.take().map(|s| s.add_num(num));
    }

    pub fn find_median(&self) -> f64 {
        self.state.as_ref().map(|s| s.find_median()).unwrap()
    }
}

impl Default for MedianFinder {
    fn default() -> Self {
        Self::new()
    }
}
