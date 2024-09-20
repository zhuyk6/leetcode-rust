struct Solution;

use std::collections::HashMap;

struct Solver {
    nums: Vec<i32>,
    and_values: Vec<i32>,
    n: usize,
    m: usize,
    memo: Vec<HashMap<i32, i32>>,
}

impl Solver {
    fn new(nums: Vec<i32>, and_values: Vec<i32>) -> Self {
        let n = nums.len();
        let m = and_values.len();
        Self {
            nums,
            and_values,
            n,
            m,
            memo: vec![HashMap::new(); n * m],
        }
    }

    fn dfs(&mut self, i: usize, j: usize, cur: i32) -> i32 {
        if i == self.n && j == self.m {
            return 0;
        }
        if i == self.n || j == self.m {
            return i32::MAX;
        }

        let next = cur & self.nums[i];
        if next & self.and_values[j] < self.and_values[j] {
            return i32::MAX;
        }

        let key = i * self.m + j;
        if let Some(&ret) = self.memo[key].get(&cur) {
            ret
        } else {
            let mut ret = self.dfs(i + 1, j, next);
            if next == self.and_values[j] {
                ret = ret.min(
                    self.dfs(i + 1, j + 1, i32::MAX)
                        .saturating_add(self.nums[i]),
                );
            }
            self.memo[key].insert(cur, ret);
            ret
        }
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn minimum_value_sum_old(nums: Vec<i32>, and_values: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = and_values.len();

        let mut f = vec![vec![i32::MAX; m]; n];

        // first segment [l0, r0]
        {
            let mut acc = i32::MAX;
            for i in 0..n {
                acc &= nums[i];
                if acc & and_values[0] < and_values[0] {
                    break;
                }
                if acc == and_values[0] {
                    f[i][0] = nums[i];
                }
            }
        }

        for i in 0..n {
            for j in 0..(m - 1) {
                if f[i][j] < i32::MAX {
                    let t = and_values[j + 1];
                    let mut acc = i32::MAX;
                    for k in (i + 1)..n {
                        acc &= nums[k];
                        if acc & t < t {
                            break;
                        }
                        if acc == t {
                            f[k][j + 1] = f[k][j + 1].min(f[i][j] + nums[k]);
                        }
                    }
                }
            }
        }

        match f[n - 1][m - 1] {
            i32::MAX => -1,
            v => v,
        }
    }

    pub fn minimum_value_sum(nums: Vec<i32>, and_values: Vec<i32>) -> i32 {
        let mut sol = Solver::new(nums, and_values);

        match sol.dfs(0, 0, i32::MAX) {
            i32::MAX => -1,
            v => v,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![1, 4, 3, 3, 2];
        let and_values = vec![0, 3, 3, 2];
        assert_eq!(Solution::minimum_value_sum(nums, and_values), 12);
    }

    #[test]
    fn sample2() {
        let nums = vec![2, 3, 5, 7, 7, 7, 5];
        let and_values = vec![0, 7, 5];
        assert_eq!(Solution::minimum_value_sum(nums, and_values), 17);
    }

    #[test]
    fn sample3() {
        let nums = vec![1, 2, 3, 4];
        let and_values = vec![2];
        assert_eq!(Solution::minimum_value_sum(nums, and_values), -1);
    }
}
