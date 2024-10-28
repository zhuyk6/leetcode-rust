pub struct Solution;

use std::collections::HashMap;

struct Dfs {
    nums: Vec<i32>,
    memo: HashMap<(usize, i32, bool), i32>,
}

impl Dfs {
    fn new(nums: Vec<i32>) -> Self {
        Self {
            nums,
            memo: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    fn dfs_with_history(&self, x: usize, mask: i32, is_max: bool, history: &mut [i32]) -> i32 {
        if x == self.nums.len() {
            // println!("final mask = {mask:10b}");
            println!("history: {history:?}");
            return if mask > 1 { 1 } else { 0 };
        }

        let mut ret = 0;

        // 前导达到上限
        if is_max {
            if self.nums[x] > 0 {
                // 0
                if mask == 0 {
                    // 前导都是 0
                    history[x] = 0;
                    ret += self.dfs_with_history(x + 1, 0, false, history);
                } else if mask & 1 == 0 {
                    // 前导不包含 0
                    history[x] = 0;
                    ret += self.dfs_with_history(x + 1, mask | 1, false, history);
                }
                // 1..nums[x]-1
                for i in 1..self.nums[x] {
                    if mask & (1 << i) == 0 {
                        history[x] = i;
                        ret += self.dfs_with_history(x + 1, mask | (1 << i), false, history);
                    }
                }
                // nums[x]
                if mask & (1 << self.nums[x]) == 0 {
                    history[x] = self.nums[x];
                    ret += self.dfs_with_history(x + 1, mask | (1 << self.nums[x]), true, history);
                }
            } else if mask & 1 == 0 {
                history[x] = 0;
                ret += self.dfs_with_history(x + 1, mask | 1, true, history);
            }
        } else {
            // 0
            if mask == 0 {
                history[x] = 0;
                ret += self.dfs_with_history(x + 1, 0, false, history);
            } else if mask & 1 == 0 {
                history[x] = 0;
                ret += self.dfs_with_history(x + 1, mask | 1, false, history);
            }

            // 1..9
            for i in 1..10 {
                if mask & (1 << i) == 0 {
                    history[x] = i;
                    ret += self.dfs_with_history(x + 1, mask | (1 << i), false, history);
                }
            }
        }

        ret
    }

    fn dfs(&mut self, x: usize, mask: i32, is_max: bool) -> i32 {
        if x == self.nums.len() {
            return if mask > 1 { 1 } else { 0 };
        }

        if let Some(&v) = self.memo.get(&(x, mask, is_max)) {
            return v;
        }

        let mut ret = 0;

        // 前导达到上限
        if is_max {
            if self.nums[x] > 0 {
                // 0
                if mask == 0 {
                    ret += self.dfs(x + 1, 0, false);
                } else if mask & 1 == 0 {
                    ret += self.dfs(x + 1, mask | 1, false);
                }
                // 1..nums[x]-1
                for i in 1..self.nums[x] {
                    if mask & (1 << i) == 0 {
                        ret += self.dfs(x + 1, mask | (1 << i), false);
                    }
                }
                // nums[x]
                if mask & (1 << self.nums[x]) == 0 {
                    ret += self.dfs(x + 1, mask | (1 << self.nums[x]), true);
                }
            } else if mask & 1 == 0 {
                ret += self.dfs(x + 1, mask | 1, true);
            }
        } else {
            // 0
            if mask == 0 {
                ret += self.dfs(x + 1, 0, false);
            } else if mask & 1 == 0 {
                ret += self.dfs(x + 1, mask | 1, false);
            }

            // 1..9
            for i in 1..10 {
                if mask & (1 << i) == 0 {
                    ret += self.dfs(x + 1, mask | (1 << i), false);
                }
            }
        }

        self.memo.insert((x, mask, is_max), ret);
        ret
    }
}

impl Solution {
    pub fn count_special_numbers(mut n: i32) -> i32 {
        let mut nums = Vec::new();
        while n > 0 {
            nums.push(n % 10);
            n /= 10;
        }
        nums.reverse();
        // dbg!(&nums);

        let mut sol = Dfs::new(nums.clone());
        // let mut history = vec![0; nums.len()];
        // sol.dfs_with_history(0, 0, true, &mut history)
        sol.dfs(0, 0, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let n = 20;
        assert_eq!(Solution::count_special_numbers(n), 19);
    }

    #[test]
    fn sample2() {
        let n = 5;
        assert_eq!(Solution::count_special_numbers(n), 5);
    }

    #[test]
    fn sample3() {
        let n = 135;
        assert_eq!(Solution::count_special_numbers(n), 110);
    }

    #[test]
    fn wrong1() {
        let n = 403;
        assert_eq!(Solution::count_special_numbers(n), 309);
    }

    #[test]
    fn wrong2() {
        let n = 500;
        assert_eq!(Solution::count_special_numbers(n), 378);
    }
}
