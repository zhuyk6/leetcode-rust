pub struct Solution;

use std::collections::HashMap;

struct Dfs {
    nums: Vec<i32>,
    mem: HashMap<(i32, i32, i32), bool>,
}

impl Dfs {
    fn with_nums(nums: Vec<i32>) -> Self {
        let mem = HashMap::new();
        Self { nums, mem }
    }

    fn dfs(&mut self, turn: bool, i: i32, j: i32, acc: i32) -> bool {
        if i > j {
            return acc >= 0;
        }
        if let Some(ans) = self.mem.get(&(i, j, acc)) {
            return *ans;
        }
        if turn {
            let mut ans = false;
            if self.dfs(!turn, i + 1, j, acc + self.nums[i as usize]) {
                ans = true;
            }
            if self.dfs(!turn, i, j - 1, acc + self.nums[j as usize]) {
                ans = true;
            }
            self.mem.insert((i, j, acc), ans);
            ans
        } else {
            let mut ans = true;
            if !self.dfs(!turn, i + 1, j, acc - self.nums[i as usize]) {
                ans = false;
            }
            if !self.dfs(!turn, i, j - 1, acc - self.nums[j as usize]) {
                ans = false;
            }
            self.mem.insert((i, j, acc), ans);
            ans
        }
    }
}

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len() as i32;
        let mut dfs = Dfs::with_nums(nums);
        dfs.dfs(true, 0, n - 1, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![1, 5, 2];
        assert!(!Solution::predict_the_winner(nums));
    }

    #[test]
    fn sample2() {
        let nums = vec![1, 5, 233, 7];
        assert!(Solution::predict_the_winner(nums));
    }

    #[test]
    fn issue() {
        let nums = vec![1, 5, 2, 4, 6];
        assert!(Solution::predict_the_winner(nums));
    }
}
