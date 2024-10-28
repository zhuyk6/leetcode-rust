pub struct Solution;

use std::collections::{HashMap, HashSet};

struct Dfs {
    nums: Vec<i32>,
    k: usize,
    m: usize,
    ans: i32,
    g: Vec<HashSet<i32>>,
}

impl Dfs {
    fn new(mut nums: Vec<i32>, k: usize) -> Self {
        nums.sort_unstable();
        let m = nums.len() / k;
        Dfs {
            nums,
            k,
            m,
            ans: i32::MAX,
            g: vec![HashSet::with_capacity(16); k],
        }
    }

    fn calc(&self) -> i32 {
        self.g
            .iter()
            .map(|s| s.iter().max().unwrap() - s.iter().min().unwrap())
            .sum()
    }

    fn dfs(&mut self, x: usize) {
        if x >= self.nums.len() {
            self.ans = self.ans.min(self.calc());
            return;
        }
        for i in 0..self.k {
            if !self.g[i].contains(&self.nums[x]) && self.g[i].len() < self.m {
                self.g[i].insert(self.nums[x]);
                self.dfs(x + 1);
                self.g[i].remove(&self.nums[x]);
            }
        }
    }
}

impl Solution {
    pub fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        assert!(nums.len() % k == 0);

        let mut cnt = HashMap::new();
        for &v in &nums {
            *cnt.entry(v).or_insert(0usize) += 1;
        }

        if *cnt.values().max().unwrap() > k {
            -1
        } else {
            let mut solver = Dfs::new(nums, k);
            solver.dfs(0);
            solver.ans
        }
    }
}

#[test]
fn test1() {
    let nums = vec![1, 2, 1, 4];
    let k = 2;
    assert_eq!(Solution::minimum_incompatibility(nums, k), 4);
}

#[test]
fn test2() {
    let nums = vec![6, 3, 8, 1, 3, 1, 2, 2];
    let k = 4;
    assert_eq!(Solution::minimum_incompatibility(nums, k), 6);
}

#[test]
fn test3() {
    let nums = vec![5, 3, 3, 6, 3, 3];
    let k = 3;
    assert_eq!(Solution::minimum_incompatibility(nums, k), -1);
}

// #[test]
// fn test4() {
//     let nums = vec![7, 3, 16, 15, 1, 13, 1, 2, 14, 5, 3, 10, 6, 2, 7, 15];
//     let k = 8;
//     assert_eq!(Solution::minimum_incompatibility(nums, k), -1);
// }
