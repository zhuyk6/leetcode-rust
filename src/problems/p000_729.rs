use std::collections::BTreeMap;

pub struct MyCalendar {
    left_set: BTreeMap<i32, i32>,
}

impl MyCalendar {
    pub fn new() -> Self {
        MyCalendar {
            left_set: BTreeMap::new(),
        }
    }

    pub fn book(&mut self, start_time: i32, end_time: i32) -> bool {
        let mut left = self.left_set.range(..=start_time);
        let mut right = self.left_set.range(start_time..);
        if left.next_back().map_or(true, |(_, &r)| r <= start_time)
            && right.next().map_or(true, |(&l, _)| l >= end_time)
        {
            self.left_set.insert(start_time, end_time);
            true
        } else {
            false
        }
    }
}

impl Default for MyCalendar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let mut calendar = MyCalendar::new();
        assert!(calendar.book(10, 20));
        assert!(!calendar.book(15, 25));
        assert!(calendar.book(20, 30));
    }
}
