pub struct Solution;

use std::collections::HashMap;

const MOD: i32 = 1_000_000_007;

#[inline]
fn mod_add(a: i32, b: i32) -> i32 {
    (a + b) % MOD
}

fn get_fail(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut fail: Vec<usize> = vec![0; n];

    for i in 1..n - 1 {
        let mut j = fail[i];
        while j > 0 && s[i] != s[j] {
            j = fail[j];
        }
        fail[i + 1] = if s[i] == s[j] { j + 1 } else { 0 };
    }

    fail
}

struct Solver<'a> {
    /// length of s1 and s2
    n: usize,

    /// length of evil
    m: usize,

    s1: &'a [u8],
    s2: &'a [u8],
    evil: &'a [u8],

    fail: Vec<usize>,

    mem: HashMap<(usize, bool, bool, usize), i32>,
}

impl<'a> Solver<'a> {
    fn new(s1: &'a [u8], s2: &'a [u8], evil: &'a [u8]) -> Self {
        let n = s1.len();
        let m = evil.len();

        let fail = get_fail(evil);

        Self {
            n,
            m,
            s1,
            s2,
            evil,
            fail,
            mem: HashMap::new(),
        }
    }

    fn dfs(
        &mut self,
        pos: usize,
        greater_than_s1: bool,
        less_than_s2: bool,
        match_evil: usize,
    ) -> i32 {
        if match_evil == self.m {
            return 0;
        }

        if pos == self.n {
            return 1;
        }

        if let Some(ret) = self
            .mem
            .get(&(pos, greater_than_s1, less_than_s2, match_evil))
        {
            return *ret;
        }

        let mut ret = 0;

        for ch in b'a'..=b'z' {
            if !greater_than_s1 && ch < self.s1[pos] {
                continue;
            }

            if !less_than_s2 && ch > self.s2[pos] {
                continue;
            }

            let mut i = match_evil;
            while i > 0 && self.evil[i] != ch {
                i = self.fail[i];
            }

            let tmp = self.dfs(
                pos + 1,
                greater_than_s1 || ch > self.s1[pos],
                less_than_s2 || ch < self.s2[pos],
                if ch == self.evil[i] { i + 1 } else { 0 },
            );

            ret = mod_add(ret, tmp);
        }

        self.mem
            .insert((pos, greater_than_s1, less_than_s2, match_evil), ret);
        ret
    }
}

impl Solution {
    pub fn find_good_strings(_n: i32, s1: String, s2: String, evil: String) -> i32 {
        let mut sol = Solver::new(s1.as_bytes(), s2.as_bytes(), evil.as_bytes());

        sol.dfs(0, false, false, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let n = 2;
        let s1 = "aa".to_string();
        let s2 = "da".to_string();
        let evil = "b".to_string();

        assert_eq!(Solution::find_good_strings(n, s1, s2, evil), 51);
    }

    #[test]
    fn sample2() {
        let n = 8;
        let s1 = "leetcode".to_string();
        let s2 = "leetgoes".to_string();
        let evil = "leet".to_string();

        assert_eq!(Solution::find_good_strings(n, s1, s2, evil), 0);
    }

    #[test]
    fn sample3() {
        let n = 2;
        let s1 = "gx".to_string();
        let s2 = "gz".to_string();
        let evil = "x".to_string();

        assert_eq!(Solution::find_good_strings(n, s1, s2, evil), 2);
    }

    #[test]
    fn issue() {
        let n = 8;
        let s1 = "pzdanyao".to_string();
        let s2 = "wgpmtywi".to_string();
        let evil = "sdka".to_string();

        assert_eq!(Solution::find_good_strings(n, s1, s2, evil), 500543753);
    }
}
