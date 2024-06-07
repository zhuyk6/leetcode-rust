struct Solution;

use std::collections::HashMap;

struct MemDfs {
    map: HashMap<i32, i32>,
}

impl MemDfs {
    fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(0, 0);
        map.insert(1, 1);

        MemDfs { map }
    }

    fn dfs(&mut self, n: i32) -> i32 {
        if let Some(&ans) = self.map.get(&n) {
            ans
        } else {
            let ans1 = n % 2 + 1 + self.dfs(n / 2);
            let ans2 = n % 3 + 1 + self.dfs(n / 3);
            let ans = i32::min(ans1, ans2);
            self.map.insert(n, ans);
            ans
        }
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn min_days(n: i32) -> i32 {
        let mut mem_dfs = MemDfs::new();
        mem_dfs.dfs(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let n = 10;
        assert_eq!(Solution::min_days(n), 4);
    }

    #[test]
    fn sample2() {
        let n = 6;
        assert_eq!(Solution::min_days(n), 3);
    }

    #[test]
    fn sample3() {
        let n = 1;
        assert_eq!(Solution::min_days(n), 1);
    }

    #[test]
    fn sample4() {
        let n = 56;
        assert_eq!(Solution::min_days(n), 6);
    }

    #[test]
    fn sample5() {
        let n = 100;
        assert_eq!(Solution::min_days(n), 8);
    }

    #[test]
    fn sample6() {
        let n = 100156287;
        assert_eq!(Solution::min_days(n), 27);
    }
}
