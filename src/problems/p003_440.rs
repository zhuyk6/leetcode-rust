pub struct Solution;

use std::collections::BTreeMap;

struct MultiSet {
    map: BTreeMap<i32, u32>,
}

impl MultiSet {
    fn new() -> Self {
        MultiSet {
            map: BTreeMap::new(),
        }
    }

    fn insert(&mut self, value: i32) {
        *self.map.entry(value).or_insert(0) += 1;
    }

    fn remove(&mut self, value: i32) {
        if let Some(count) = self.map.get_mut(&value) {
            if *count > 1 {
                *count -= 1;
            } else {
                self.map.remove(&value);
            }
        }
    }

    fn max(&self) -> Option<i32> {
        self.map.keys().next_back().copied()
    }
}

impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut spaces: Vec<i32> = vec![start_time[0]];
        for i in 1..=n {
            spaces.push(start_time.get(i).copied().unwrap_or(event_time) - end_time[i - 1]);
        }
        let spaces = spaces;
        // println!("{spaces:#?}");

        let lengths = start_time
            .iter()
            .zip(&end_time)
            .map(|(&s, &t)| t - s)
            .collect::<Vec<i32>>();

        let mut ans = 0;
        let mut multiset = MultiSet::new();
        for &s in &spaces {
            multiset.insert(s);
        }

        for i in 0..n {
            multiset.remove(spaces[i]);
            multiset.remove(spaces[i + 1]);

            if multiset
                .max()
                .and_then(|second_max| {
                    if second_max >= lengths[i] {
                        Some(second_max)
                    } else {
                        None
                    }
                })
                .is_some()
            {
                ans = ans.max(spaces[i] + spaces[i + 1] + lengths[i]);
            } else {
                ans = ans.max(spaces[i] + spaces[i + 1]);
            }

            multiset.insert(spaces[i]);
            multiset.insert(spaces[i + 1]);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let event_time = 5;
        let start_time = vec![1, 3];
        let end_time = vec![2, 5];
        assert_eq!(Solution::max_free_time(event_time, start_time, end_time), 2);
    }

    #[test]
    fn sample2() {
        let event_time = 10;
        let start_time = vec![0, 7, 9];
        let end_time = vec![1, 8, 10];
        assert_eq!(Solution::max_free_time(event_time, start_time, end_time), 7);
    }

    #[test]
    fn sample3() {
        let event_time = 10;
        let start_time = vec![0, 3, 7, 9];
        let end_time = vec![1, 4, 8, 10];
        assert_eq!(Solution::max_free_time(event_time, start_time, end_time), 6);
    }

    #[test]
    fn sample4() {
        let event_time = 5;
        let start_time = vec![0, 1, 2, 3, 4];
        let end_time = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::max_free_time(event_time, start_time, end_time), 0);
    }
}
