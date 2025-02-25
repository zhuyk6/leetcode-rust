pub mod segment_tree {
    #[derive(Debug)]
    enum Node {
        Leaf {
            #[allow(unused)]
            pos: i32,
            count: u32,
        },
        Internal {
            left: i32,
            right: i32,
            lazy_count: u32,
            max_count: u32,
            lc: Option<Box<Node>>,
            rc: Option<Box<Node>>,
        },
    }

    impl Node {
        fn with(left: i32, right: i32, count: u32) -> Self {
            match left.cmp(&right) {
                std::cmp::Ordering::Less => Node::Internal {
                    left,
                    right,
                    lazy_count: count,
                    max_count: count,
                    lc: None,
                    rc: None,
                },
                std::cmp::Ordering::Equal => Node::Leaf { pos: left, count },
                std::cmp::Ordering::Greater => panic!("left must be less than right"),
            }
        }

        fn insert(&mut self, start: i32, end: i32, delta: u32) {
            match self {
                Node::Leaf { count, .. } => {
                    *count += delta;
                }
                &mut Node::Internal {
                    ref left,
                    ref right,
                    ref mut lazy_count,
                    ref mut max_count,
                    ref mut lc,
                    ref mut rc,
                } => {
                    if start <= *left && *right <= end {
                        *lazy_count += delta;
                        *max_count += delta;
                        return;
                    }
                    let mid = (left + right) / 2;

                    // pushdown lazy_count
                    let lc = lc.get_or_insert_with(|| Box::new(Node::with(*left, mid, 0)));
                    let rc = rc.get_or_insert_with(|| Box::new(Node::with(mid + 1, *right, 0)));

                    lc.insert(*left, mid, *lazy_count);
                    rc.insert(mid + 1, *right, *lazy_count);
                    *lazy_count = 0;

                    // insert [start, end]
                    if start <= mid {
                        lc.insert(start, end, delta);
                    }
                    if mid < end {
                        rc.insert(start, end, delta);
                    }
                    // update max_count
                    *max_count = lc.max_count().max(rc.max_count());
                }
            }
        }

        fn max_count(&self) -> u32 {
            match self {
                Node::Leaf { count, .. } => *count,
                Node::Internal { max_count, .. } => *max_count,
            }
        }

        fn query(&mut self, start: i32, end: i32) -> u32 {
            match self {
                Node::Leaf { count, .. } => *count,
                &mut Node::Internal {
                    ref left,
                    ref right,
                    ref mut lc,
                    ref mut rc,
                    ref mut max_count,
                    ref mut lazy_count,
                } => {
                    if start <= *left && *right <= end {
                        return *max_count;
                    }
                    let mid = (*left + *right) / 2;

                    // pushdown lazy_count
                    let lc = lc.get_or_insert_with(|| Box::new(Node::with(*left, mid, 0)));
                    let rc = rc.get_or_insert_with(|| Box::new(Node::with(mid + 1, *right, 0)));
                    lc.insert(*left, mid, *lazy_count);
                    rc.insert(mid + 1, *right, *lazy_count);
                    *lazy_count = 0;

                    // query [start, end]
                    let mut res = 0;
                    if start <= mid {
                        res = res.max(lc.query(start, end));
                    }
                    if mid < end {
                        res = res.max(rc.query(start, end));
                    }
                    res
                }
            }
        }
    }

    pub struct MyCalendarTwo {
        root: Box<Node>,
    }

    impl MyCalendarTwo {
        pub fn new() -> Self {
            MyCalendarTwo {
                root: Box::new(Node::with(0, 1_000_000_000, 0)),
            }
        }

        pub fn book(&mut self, start_time: i32, end_time: i32) -> bool {
            if self.root.query(start_time, end_time - 1) < 2 {
                self.root.insert(start_time, end_time - 1, 1);
                true
            } else {
                false
            }
        }
    }

    impl Default for MyCalendarTwo {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub struct MyCalendarTwo {
    left_set1: std::collections::BTreeMap<i32, i32>,
    left_set2: std::collections::BTreeMap<i32, i32>,
}

impl MyCalendarTwo {
    pub fn new() -> Self {
        MyCalendarTwo {
            left_set1: std::collections::BTreeMap::new(),
            left_set2: std::collections::BTreeMap::new(),
        }
    }

    pub fn book(&mut self, mut start_time: i32, end_time: i32) -> bool {
        let mut left = self.left_set2.range(..start_time);
        let mut right = self.left_set2.range(start_time..);
        if left.next_back().is_none_or(|(_, &r)| r <= start_time)
            && right.next().is_none_or(|(&l, _)| l >= end_time)
        {
            let mut v_delete = vec![];
            let mut v1 = vec![];
            let mut v2 = vec![];
            let mut left = self.left_set1.range(..start_time);
            if let Some((&l, &r)) = left.next_back() {
                if r > start_time {
                    v_delete.push((l, r));
                    v1.push((l, start_time));

                    if end_time < r {
                        v2.push((start_time, end_time));
                        v1.push((end_time, r));
                        start_time = end_time;
                    } else {
                        v2.push((start_time, r));
                        start_time = r;
                    }
                }
            }

            let right = self.left_set1.range(start_time..);
            for (&l, &r) in right {
                if l >= end_time {
                    break;
                }
                v_delete.push((l, r));
                if l > start_time {
                    v1.push((start_time, l));
                }
                if r <= end_time {
                    v2.push((l, r));
                    start_time = r;
                } else {
                    v2.push((l, end_time));
                    v1.push((end_time, r));
                    start_time = end_time;
                }
            }

            if start_time < end_time {
                v1.push((start_time, end_time));
            }

            for (l, _) in v_delete {
                self.left_set1.remove(&l);
            }

            for (l, r) in v1 {
                self.left_set1.insert(l, r);
            }

            for (l, r) in v2 {
                self.left_set2.insert(l, r);
            }

            // dbg!(&self.left_set1, &self.left_set2);

            true
        } else {
            false
        }
    }
}

impl Default for MyCalendarTwo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        // let mut calendar = segment_tree::MyCalendarTwo::new();
        let mut calendar = MyCalendarTwo::new();
        assert!(calendar.book(10, 20));
        assert!(calendar.book(50, 60));
        assert!(calendar.book(10, 40));
        assert!(!calendar.book(5, 15));
        assert!(calendar.book(5, 10));
        assert!(calendar.book(25, 55));
    }

    #[test]
    fn issue() {
        let qs = [
            [28, 46],
            [9, 21],
            [21, 39],
            [37, 48],
            [38, 50],
            [22, 39],
            [45, 50],
            [1, 12],
            [40, 50],
            [31, 44],
        ];
        let ans = [
            true, true, true, false, false, false, true, true, false, false,
        ];
        let mut calendar = MyCalendarTwo::new();
        for (q, a) in qs.iter().zip(ans) {
            assert_eq!(calendar.book(q[0], q[1]), a);
        }
    }

    #[test]
    fn issue2() {
        let qs = [
            [89, 100],
            [30, 43],
            [92, 100],
            [31, 49],
            [59, 76],
            [60, 73],
            [31, 49],
            [80, 99],
            [48, 60],
            [36, 52],
            [67, 82],
            [96, 100],
            [22, 35],
            [18, 32],
            [9, 24],
            [11, 27],
            [94, 100],
            [12, 22],
            [61, 74],
            [3, 20],
            [14, 28],
            [27, 37],
            [5, 20],
            [1, 11],
            [96, 100],
            [33, 44],
            [90, 100],
            [40, 54],
            [23, 35],
            [18, 32],
            [78, 89],
            [56, 66],
            [83, 93],
            [45, 59],
            [40, 59],
            [94, 100],
            [99, 100],
            [86, 96],
            [43, 61],
            [29, 45],
            [21, 36],
            [13, 31],
            [17, 30],
            [16, 30],
            [80, 94],
            [38, 50],
            [15, 30],
            [62, 79],
            [25, 39],
            [73, 85],
            [39, 56],
            [80, 97],
            [42, 57],
            [32, 47],
            [59, 78],
            [35, 53],
            [56, 74],
            [75, 85],
            [39, 54],
            [63, 82],
        ];
        let ans = [
            true, true, true, true, true, true, false, false, true, false, false, false, false,
            false, true, true, false, false, false, false, false, false, false, true, false, false,
            false, false, false, false, true, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, true,
            false, false, false, false, false, false, false, false, false, false,
        ];
        let mut calendar = MyCalendarTwo::new();
        for (q, a) in qs.iter().zip(ans) {
            assert_eq!(calendar.book(q[0], q[1]), a);
        }
    }
}
