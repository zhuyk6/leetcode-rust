pub struct Solution;

use std::collections::HashMap;

#[derive(Debug, Default)]
struct Counter {
    counter: HashMap<i32, u32>,
}

impl Counter {
    #[must_use]
    fn new() -> Self {
        Self {
            counter: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) {
        *self.counter.entry(val).or_insert(0) += 1;
    }

    fn remove(&mut self, val: i32) {
        if let Some(count) = self.counter.get_mut(&val) {
            *count -= 1;
            if *count == 0 {
                self.counter.remove(&val);
            }
        }
    }

    fn different_count(&self) -> usize {
        self.counter.len()
    }
}

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let different_count = {
            let mut counter = Counter::new();
            for &num in &nums {
                counter.insert(num);
            }
            counter.different_count()
        };

        let mut ans = 0;
        let n = nums.len();
        let mut counter = Counter::new();
        let mut i = 0;
        let mut j = 0;
        while i < n {
            while j < n && counter.different_count() < different_count {
                counter.insert(nums[j]);
                j += 1;
            }
            if counter.different_count() == different_count {
                ans += n - j + 1;
            }
            counter.remove(nums[i]);
            i += 1;
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![1, 3, 1, 2, 2];
        assert_eq!(Solution::count_complete_subarrays(nums), 4);
    }

    #[test]
    fn sample2() {
        let nums = vec![5, 5, 5, 5];
        assert_eq!(Solution::count_complete_subarrays(nums), 10);
    }
}
