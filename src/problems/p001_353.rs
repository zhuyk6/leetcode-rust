pub struct Solution;

impl Solution {
    pub fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort_unstable_by_key(|e| (e[0], e[1]));

        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut min_heap = BinaryHeap::new();

        let mut ans = 0;

        let n = events.len();
        let start_day = events[0][0];
        let end_day = events.iter().map(|e| e[1]).max().unwrap_or(start_day);

        let mut e_id = 0;
        for day in start_day..=end_day {
            while e_id < n && events[e_id][0] == day {
                min_heap.push(Reverse(events[e_id][1]));
                e_id += 1;
            }

            while let Some(Reverse(end)) = min_heap.pop() {
                if end >= day {
                    ans += 1;
                    break;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let events = nested_vec![[1, 2], [2, 3], [3, 4]];
        assert_eq!(Solution::max_events(events), 3);
    }

    #[test]
    fn sample2() {
        let events = nested_vec![[1, 2], [2, 3], [3, 4], [1, 2]];
        assert_eq!(Solution::max_events(events), 4);
    }

    #[test]
    fn issue() {
        let events = nested_vec![[1, 2], [1, 2], [1, 6], [1, 2], [1, 2]];
        assert_eq!(Solution::max_events(events), 3);
    }
}
