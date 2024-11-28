pub struct Solution;

use std::collections::HashMap;

#[inline]
fn check_sqr(x: i32) -> bool {
    let y = (x as f64).sqrt().floor() as i32;
    y.pow(2) == x || (y + 1).pow(2) == x
}

struct Dfs {
    n: usize,
    nums: Vec<(i32, u32)>,
    ans: i32,
}

impl Dfs {
    fn with_nums(nums: Vec<i32>) -> Self {
        let n = nums.len();

        let mut cnt: HashMap<i32, u32> = HashMap::new();
        for v in nums {
            *cnt.entry(v).or_default() += 1;
        }

        let nums: Vec<(i32, u32)> = cnt.into_iter().collect();

        Self { n, nums, ans: 0 }
    }

    fn dfs(&mut self, i: usize, last: Option<i32>) {
        if i == self.n {
            self.ans += 1;
        } else {
            for idx in 0..self.nums.len() {
                let (v, left) = self.nums[idx];
                if left > 0
                    && last
                        .as_ref()
                        .map(|&last| last != v && check_sqr(v + last))
                        .unwrap_or(true)
                {
                    if left > 1 && check_sqr(v + v) {
                        for used in (2..=left).rev() {
                            self.nums[idx].1 -= used;
                            self.dfs(i + used as usize, Some(v));
                            self.nums[idx].1 += used;
                        }
                    }
                    // used = 1
                    self.nums[idx].1 -= 1;
                    self.dfs(i + 1, Some(v));
                    self.nums[idx].1 += 1;
                }
            }
        }
    }
}

impl Solution {
    pub fn num_squareful_perms(nums: Vec<i32>) -> i32 {
        let mut sol = Dfs::with_nums(nums);
        sol.dfs(0, None);
        sol.ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issue() {
        let nums = vec![1, 1, 8, 1, 8];
        assert_eq!(Solution::num_squareful_perms(nums), 1);
    }
}
