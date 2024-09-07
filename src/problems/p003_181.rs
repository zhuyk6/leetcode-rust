struct Solution;

struct Solver {
    nums: Vec<i32>,
    f: Vec<bool>,
    last: usize,
}

#[allow(dead_code)]
impl Solver {
    fn new(nums: Vec<i32>) -> Self {
        let last = *nums.last().unwrap();
        let f = vec![false; 50_000 + 1];
        Self {
            nums,
            f,
            last: last as usize,
        }
    }

    fn dfs(&mut self, x: usize) {
        if x >= self.last || self.f[x] {
            return;
        }
        self.f[x] = true;
        let p = self.nums.partition_point(|v| *v <= x as i32);
        for i in p..self.nums.len() {
            self.dfs(x + self.nums[i] as usize);
        }
    }

    fn answer(&self) -> usize {
        for i in (0..self.last).rev() {
            if self.f[i] {
                return i;
            }
        }
        0
    }
}

struct Solver2 {
    nums: Vec<i32>,
    f: Vec<Option<bool>>,
}

#[allow(dead_code)]
impl Solver2 {
    fn new(nums: Vec<i32>) -> Self {
        let f = vec![None; 50_000 + 1];
        Self { nums, f }
    }

    fn query(&mut self, x: usize) -> bool {
        if x == 0 {
            true
        } else {
            if self.f[x].is_some() {
                return self.f[x].unwrap();
            }
            let p = self.nums.partition_point(|&v| v as usize <= x / 2);
            for i in p..self.nums.len() {
                if self.nums[i] as usize > x {
                    break;
                }
                let y = x - self.nums[i] as usize;
                if self.query(y) {
                    self.f[x] = Some(true);
                    return true;
                }
            }
            self.f[x] = Some(false);
            false
        }
    }

    fn answer(&mut self) -> i32 {
        let n = self.nums.len();
        match n {
            0 => panic!("impossible!"),
            1 => self.nums[0],
            _ => {
                let z = self.nums[n - 1];
                let y = self.nums[n - 2];
                for x in (y..z).rev() {
                    if self.query(x as usize) {
                        return x + z;
                    }
                }
                y + z
            }
        }
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn max_total_reward1(reward_values: Vec<i32>) -> i32 {
        let mut nums = reward_values;
        nums.sort();
        nums.dedup();

        let mut sol = Solver::new(nums);
        sol.dfs(0);

        (sol.last + sol.answer()) as i32
    }

    pub fn max_total_reward(reward_values: Vec<i32>) -> i32 {
        let mut nums = reward_values;
        nums.sort();
        nums.dedup();

        let mut sol = Solver2::new(nums);
        sol.answer()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let reward_values = vec![1, 1, 3, 3];
        assert_eq!(Solution::max_total_reward(reward_values), 4);
    }

    #[test]
    fn sample2() {
        let reward_values = vec![1, 6, 4, 3, 2];
        assert_eq!(Solution::max_total_reward(reward_values), 11);
    }
}
