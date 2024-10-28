use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap};
use std::ops::Bound::Excluded;

#[derive(Debug, PartialEq, Eq)]
struct Segment {
    l: i32,
    r: i32,
}

impl PartialOrd for Segment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Segment {
    fn cmp(&self, other: &Self) -> Ordering {
        let len1 = (self.r - self.l) / 2;
        let len2 = (other.r - other.l) / 2;
        match len1.cmp(&len2) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                if self.l < other.l {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        }
    }
}

pub struct ExamRoom {
    num: i32,
    seats: BTreeSet<i32>,
    segments: BinaryHeap<Segment>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl ExamRoom {
    pub fn new(n: i32) -> Self {
        ExamRoom {
            num: n,
            seats: BTreeSet::new(),
            segments: BinaryHeap::new(),
        }
    }

    pub fn seat(&mut self) -> i32 {
        // println!("{:#?}", self.seats);
        // println!("{:#?}", self.segments);

        let n = self.seats.len();
        if n == 0 {
            self.seats.insert(0);
            0
        } else if n == 1 {
            // let left = *self.seats.first().unwrap();
            let left = *self.seats.iter().next().unwrap();
            let right = self.num - 1 - left;
            if left >= right {
                self.seats.insert(0);
                self.segments.push(Segment { l: 0, r: left });
                0
            } else {
                self.seats.insert(self.num - 1);
                self.segments.push(Segment {
                    l: left,
                    r: self.num - 1,
                });
                self.num - 1
            }
        } else {
            // let first = *self.seats.first().unwrap();
            // let last = *self.seats.last().unwrap();
            let first = *self.seats.iter().next().unwrap();
            let last = *self.seats.iter().next_back().unwrap();
            let left = first;
            let right = self.num - 1 - last;

            while let Some(segment) = self.segments.peek() {
                // check valid
                if !self.seats.contains(&segment.l)
                    || !self.seats.contains(&segment.r)
                    || self
                        .seats
                        .range((Excluded(segment.l), Excluded(segment.r)))
                        .next()
                        .is_some()
                {
                    self.segments.pop();
                    continue;
                }

                let len = (segment.r - segment.l) / 2;
                if right > left && right > len {
                    self.seats.insert(self.num - 1);
                    self.segments.push(Segment {
                        l: last,
                        r: self.num - 1,
                    });
                    return self.num - 1;
                } else if left >= right && left >= len {
                    self.seats.insert(0);
                    self.segments.push(Segment { l: 0, r: first });
                    return 0;
                } else {
                    let Segment { l, r } = *segment;
                    self.segments.pop();
                    let p = l + len;
                    self.seats.insert(p);
                    self.segments.push(Segment { l, r: p });
                    self.segments.push(Segment { l: p, r });
                    return p;
                }
            }
            -1
        }
    }

    pub fn leave(&mut self, p: i32) {
        self.seats.remove(&p);
        let prev = self.seats.range(..p).next_back();
        let next = self.seats.range(p..).next();
        if let Some(x) = prev {
            if let Some(y) = next {
                self.segments.push(Segment { l: *x, r: *y });
            }
        }
    }
}

/**
 * Your ExamRoom object will be instantiated and called as such:
 * let obj = ExamRoom::new(n);
 * let ret_1: i32 = obj.seat();
 * obj.leave(p);
 */

#[cfg(test)]
mod tests {
    use super::{ExamRoom, Segment};

    #[test]
    fn test_segment() {
        let seg1 = Segment { l: 4, r: 9 };
        let seg2 = Segment { l: 0, r: 4 };
        assert!(seg1 < seg2);
    }

    #[test]
    fn test1() {
        let input1 = ["ExamRoom", "seat", "seat", "seat", "seat", "leave", "seat"];
        let input2 = [[10], [-1], [-1], [-1], [-1], [4], [-1]];
        let output = [-1, 0, 9, 4, 2, -1, 5];

        let mut room = ExamRoom::new(input2[0][0]);
        for i in 1..input1.len() {
            match input1[i] {
                "seat" => {
                    let ans = room.seat();
                    println!("seat at {}", ans);
                    assert_eq!(ans, output[i]);
                }
                "leave" => {
                    room.leave(input2[i][0]);
                    println!("leave at {}", input2[i][0]);
                }
                _ => {
                    panic!("Error! Invalid operation.");
                }
            }
        }
    }

    #[test]
    fn test2() {
        let input1 = [
            "ExamRoom", "seat", "seat", "seat", "leave", "leave", "seat", "seat", "seat", "seat",
            "seat", "seat", "seat", "seat", "seat", "leave", "leave", "seat", "seat", "leave",
            "seat", "leave", "seat", "leave", "seat", "leave", "seat", "leave", "leave", "seat",
            "seat", "leave", "leave", "seat", "seat", "leave",
        ];
        let input2 = [
            [10],
            [-1],
            [-1],
            [-1],
            [0],
            [4],
            [-1],
            [-1],
            [-1],
            [-1],
            [-1],
            [-1],
            [-1],
            [-1],
            [-1],
            [0],
            [4],
            [-1],
            [-1],
            [7],
            [-1],
            [3],
            [-1],
            [3],
            [-1],
            [9],
            [-1],
            [0],
            [8],
            [-1],
            [-1],
            [0],
            [8],
            [-1],
            [-1],
            [2],
        ];
        let output = [
            -1, 0, 9, 4, -1, -1, 0, 4, 2, 6, 1, 3, 5, 7, 8, -1, -1, 0, 4, -1, 7, -1, 3, -1, 3, -1,
            9, -1, -1, 0, 8, -1, -1, 0, 8, -1,
        ];

        let mut room = ExamRoom::new(input2[0][0]);
        for i in 1..input1.len() {
            match input1[i] {
                "seat" => {
                    let ans = room.seat();
                    println!("seat at {}", ans);
                    assert_eq!(ans, output[i]);
                }
                "leave" => {
                    room.leave(input2[i][0]);
                    println!("leave at {}", input2[i][0]);
                }
                _ => {
                    panic!("Error! Invalid operation.");
                }
            }
        }
    }
}
