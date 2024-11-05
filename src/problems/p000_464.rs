pub struct Solution;

struct Dfs {
    n: usize,
    m: usize,
    mem: Vec<Vec<Option<bool>>>,
}

impl Dfs {
    fn with_size(n: usize, m: usize) -> Self {
        let mem = vec![vec![None; m + 1]; 1 << n];
        Self { n, m, mem }
    }

    fn dfs(&mut self, s: usize, acc: usize) -> bool {
        if s == 0 {
            return false;
        }
        if let Some(&ans) = self.mem[s][acc].as_ref() {
            return ans;
        }
        let mut ans = false;
        for i in 0..self.n {
            if s & (1 << i) > 0 {
                if acc + i + 1 >= self.m {
                    ans = true;
                    break;
                }
                if !self.dfs(s ^ (1 << i), acc + i + 1) {
                    ans = true;
                    break;
                }
            }
        }
        self.mem[s][acc] = Some(ans);
        ans
    }
}

impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        let n = max_choosable_integer as usize;
        let m = desired_total as usize;
        if n * (n + 1) / 2 < m {
            return false;
        }
        let mut dfs = Dfs::with_size(n, m);
        dfs.dfs((1 << n) - 1, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let max_choosable_integer = 10;
        let desired_total = 11;
        assert!(!Solution::can_i_win(max_choosable_integer, desired_total));
    }

    #[test]
    fn sample2() {
        let max_choosable_integer = 10;
        let desired_total = 0;
        assert!(Solution::can_i_win(max_choosable_integer, desired_total));
    }

    #[test]
    fn sample3() {
        let max_choosable_integer = 10;
        let desired_total = 1;
        assert!(Solution::can_i_win(max_choosable_integer, desired_total));
    }

    #[test]
    fn issue() {
        let max_choosable_integer = 5;
        let desired_total = 50;
        assert!(!Solution::can_i_win(max_choosable_integer, desired_total));
    }
}
