struct Solution;

struct Dfs<'a> {
    nums: &'a [i32],
    f: Vec<Vec<i32>>,
}

impl<'a> Dfs<'a> {
    fn new(nums: &'a [i32]) -> Self {
        let n = nums.len();
        Dfs {
            nums,
            f: vec![vec![-1; n]; n],
        }
    }

    fn dfs(&mut self, l: usize, r: usize, target: i32) -> i32 {
        if l >= r {
            return 0;
        }
        if self.f[l][r] == -1 {
            let mut ans = 0;
            if self.nums[l] + self.nums[l + 1] == target {
                ans = ans.max(1 + self.dfs(l + 2, r, target));
            }
            if self.nums[l] + self.nums[r] == target {
                ans = ans.max(1 + self.dfs(l + 1, r - 1, target));
            }
            if self.nums[r] + self.nums[r - 1] == target {
                ans = ans.max(1 + self.dfs(l, r.saturating_sub(2), target));
            }
            self.f[l][r] = ans;
        }
        self.f[l][r]
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        {
            let mut sol = Dfs::new(&nums);
            ans = ans.max(sol.dfs(0, n - 1, nums[0] + nums[1]));
        }
        {
            let mut sol = Dfs::new(&nums);
            ans = ans.max(sol.dfs(0, n - 1, nums[0] + nums[n - 1]));
        }
        {
            let mut sol = Dfs::new(&nums);
            ans = ans.max(sol.dfs(0, n - 1, nums[n - 2] + nums[n - 1]));
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong1() {
        let nums = vec![1, 1, 1, 1, 1, 1];
        assert_eq!(Solution::max_operations(nums), 3);
    }
}
