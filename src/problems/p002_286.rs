enum Node {
    Inner {
        left: usize,
        right: usize,
        max: i32,
        sum: i64,
        lc: Link,
        rc: Link,
        tag_clear: bool,
    },
    Leaf {
        pos: usize,
        val: i32,
    },
}

type Link = Box<Node>;

impl Node {
    fn build(left: usize, right: usize, val: i32) -> Box<Node> {
        match left.cmp(&right) {
            std::cmp::Ordering::Less => {
                let mid = (left + right) >> 1;
                let lc = Node::build(left, mid, val);
                let rc = Node::build(mid + 1, right, val);
                let sum = (right - left + 1) as i64 * val as i64;
                Box::new(Node::Inner {
                    left,
                    right,
                    sum,
                    lc,
                    rc,
                    max: val,
                    tag_clear: false,
                })
            }
            std::cmp::Ordering::Equal => Box::new(Node::Leaf { pos: left, val }),
            std::cmp::Ordering::Greater => panic!("left should be less than or equal to right."),
        }
    }

    fn get_max(&self) -> i32 {
        match self {
            Node::Inner { max, .. } => *max,
            Node::Leaf { val, .. } => *val,
        }
    }

    fn get_sum(&self) -> i64 {
        match self {
            Node::Inner { sum, .. } => *sum,
            Node::Leaf { val, .. } => *val as i64,
        }
    }

    fn update(&mut self) {
        if let Node::Inner {
            max, sum, lc, rc, ..
        } = self
        {
            *sum = lc.get_sum() + rc.get_sum();
            *max = i32::max(lc.get_max(), rc.get_max());
        }
    }

    fn set_tag(&mut self) {
        match self {
            Node::Inner {
                max,
                sum,
                tag_clear,
                ..
            } => {
                *tag_clear = true;
                *sum = 0;
                *max = 0;
            }
            Node::Leaf { val, .. } => {
                *val = 0;
            }
        }
    }

    // fn push_down(&mut self) {
    //     if let Node::Inner {
    //         max,
    //         sum,
    //         lc,
    //         rc,
    //         tag_clear,
    //         ..
    //     } = self
    //     {
    //         if *tag_clear {
    //             *tag_clear = false;
    //             *max = 0;
    //             *sum = 0;

    //             lc.set_tag();
    //             rc.set_tag();
    //         }
    //     }
    // }

    fn insert_left(&mut self, k: i32, max_row: usize) -> Option<(usize, i32)> {
        match self {
            Node::Inner {
                ref left,
                ref right,
                ref max,
                lc,
                rc,
                tag_clear,
                ..
            } => {
                if max_row >= *right && *max < k {
                    return None;
                }

                // push down tag
                if *tag_clear {
                    *tag_clear = false;
                    lc.set_tag();
                    rc.set_tag();
                }

                if let Some(ret) = lc.insert_left(k, max_row) {
                    self.update();
                    return Some(ret);
                }

                let mid = (*left + *right) >> 1;
                if mid < max_row {
                    let ret = rc.insert_left(k, max_row);
                    self.update();
                    return ret;
                }
                None
            }
            Node::Leaf { pos, val } => {
                if *val < k {
                    None
                } else {
                    let ret = *val;
                    *val -= k;
                    Some((*pos, ret))
                }
            }
        }
    }

    fn query_sum(&mut self, max_row: usize) -> i64 {
        match self {
            Node::Inner {
                ref left,
                ref right,
                max: _,
                ref sum,
                lc,
                rc,
                tag_clear,
            } => {
                if max_row >= *right {
                    return *sum;
                }

                // push down tag
                if *tag_clear {
                    *tag_clear = false;
                    lc.set_tag();
                    rc.set_tag();
                }

                let mut ret = 0;
                ret += lc.query_sum(max_row);

                let mid = (*left + *right) >> 1;
                if max_row > mid {
                    ret += rc.query_sum(max_row);
                }

                self.update();
                ret
            }
            Node::Leaf { pos: _, ref val } => *val as i64,
        }
    }

    fn scatter_left(&mut self, k: &mut i64) {
        if *k == 0 {
            return;
        }

        match self {
            Node::Inner {
                max,
                sum,
                lc,
                rc,
                tag_clear,
                ..
            } => {
                if *sum <= *k {
                    *k -= *sum;
                    *tag_clear = true;
                    *sum = 0;
                    *max = 0;
                    return;
                }

                // push down
                if *tag_clear {
                    *tag_clear = false;
                    lc.set_tag();
                    rc.set_tag();
                }

                lc.scatter_left(k);
                rc.scatter_left(k);

                self.update();
            }
            Node::Leaf { val, .. } => {
                if *val as i64 <= *k {
                    *k -= *val as i64;
                    *val = 0;
                } else {
                    *val -= *k as i32;
                    *k = 0;
                }
            }
        }
    }
}

#[allow(dead_code)]
struct BookMyShow {
    root: Link,
    n: i32,
    m: i32,
}

// `&self` means the method takes an immutable reference.
// If you need a mutable reference, change it to `&mut self` instead.

#[allow(dead_code)]
impl BookMyShow {
    fn new(n: i32, m: i32) -> Self {
        BookMyShow {
            root: Node::build(0, n as usize - 1, m),
            n,
            m,
        }
    }

    fn gather(&mut self, k: i32, max_row: i32) -> Vec<i32> {
        match self.root.insert_left(k, max_row as usize) {
            Some((r, c)) => vec![r as i32, self.m - c],
            None => vec![],
        }
    }

    fn scatter(&mut self, k: i32, max_row: i32) -> bool {
        if self.root.query_sum(max_row as usize) >= k as i64 {
            let mut k = k as i64;
            self.root.scatter_left(&mut k);
            true
        } else {
            false
        }
    }
}

// Your BookMyShow object will be instantiated and called as such:
// let obj = BookMyShow::new(n, m);
// let ret_1: Vec<i32> = obj.gather(k, maxRow);
// let ret_2: bool = obj.scatter(k, maxRow);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let (n, m) = (2, 5);
        let mut book = BookMyShow::new(n, m);

        assert_eq!(book.gather(4, 0), vec![0, 0]);
        assert_eq!(book.gather(2, 0), vec![]);
        assert!(book.scatter(5, 1));
        assert!(!book.scatter(5, 1));
    }
}
