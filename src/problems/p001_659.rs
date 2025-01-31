pub struct Solution;

use std::collections::HashMap;

struct Dfs {
    m: usize,
    n: usize,
    num1: usize,
    num2: usize,
    mem: HashMap<(usize, u32, usize, usize), i32>,
    cost: Vec<Vec<i32>>,
}

impl Dfs {
    fn new(m: usize, n: usize, num1: usize, num2: usize) -> Self {
        let mask = 3u32.pow(n as u32);
        let mut cost = vec![vec![0; mask as usize]; mask as usize];

        for i in 0..mask {
            for j in 0..mask {
                let mut enc1 = i;
                let mut enc2 = j;
                let mut ret = 0;
                for _ in 0..n {
                    let x1 = enc1 % 3;
                    let x2 = enc2 % 3;
                    match (x1, x2) {
                        (1, 1) => ret += -30 - 30,
                        (1, 2) | (2, 1) => ret += -30 + 20,
                        (2, 2) => ret += 20 + 20,
                        _ => {}
                    }
                    enc1 /= 3;
                    enc2 /= 3;
                }
                cost[i as usize][j as usize] = ret;
            }
        }

        Self {
            m,
            n,
            num1,
            num2,
            mem: HashMap::new(),
            cost,
        }
    }

    #[inline]
    fn calc_inline(&self, mut enc: u32) -> i32 {
        let mut ret = 0;
        let mut pre = 0;
        for _ in 0..self.n {
            let x = enc % 3;
            match x {
                0 => {}
                1 => {
                    ret += 120;
                    match pre {
                        0 => {}
                        1 => ret += -30 - 30,
                        _ => ret += -30 + 20,
                    }
                }
                _ => {
                    ret += 40;
                    match pre {
                        0 => {}
                        1 => ret += 20 - 30,
                        _ => ret += 20 + 20,
                    }
                }
            }
            pre = x;
            enc /= 3;
        }
        ret
    }

    #[inline]
    fn calc_two_lines(&self, enc1: u32, enc2: u32) -> i32 {
        self.cost[enc1 as usize][enc2 as usize]
    }

    fn decode(&self, mut enc: u32) -> (usize, usize) {
        let mut c1 = 0;
        let mut c2 = 0;
        for _ in 0..self.n {
            match enc % 3 {
                1 => c1 += 1,
                2 => c2 += 1,
                _ => {}
            }
            enc /= 3;
        }
        (c1, c2)
    }

    fn dfs(&mut self, row: usize, pre_enc: u32, left1: usize, left2: usize) -> i32 {
        if row == self.m {
            return 0;
        }

        if left1 == 0 && left2 == 0 {
            return 0;
        }

        if self.mem.contains_key(&(row, pre_enc, left1, left2)) {
            return self.mem[&(row, pre_enc, left1, left2)];
        }

        let mut ret = i32::MIN;

        // put current row
        for enc in 0u32..3u32.pow(self.n as u32) {
            let (c1, c2) = self.decode(enc);
            if c1 > left1 || c2 > left2 {
                continue;
            }

            let mut tmp = self.calc_inline(enc);
            tmp += self.calc_two_lines(enc, pre_enc);
            tmp += self.dfs(row + 1, enc, left1 - c1, left2 - c2);
            ret = ret.max(tmp);
        }

        self.mem.insert((row, pre_enc, left1, left2), ret);

        ret
    }

    fn run(&mut self) -> i32 {
        self.dfs(0, 0, self.num1, self.num2)
    }
}

impl Solution {
    pub fn get_max_grid_happiness(
        m: i32,
        n: i32,
        introverts_count: i32,
        extroverts_count: i32,
    ) -> i32 {
        let mut dfs = Dfs::new(
            m as usize,
            n as usize,
            introverts_count as usize,
            extroverts_count as usize,
        );
        dfs.run()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let m = 2;
        let n = 3;
        let introverts_count = 1;
        let extroverts_count = 2;
        assert_eq!(
            Solution::get_max_grid_happiness(m, n, introverts_count, extroverts_count),
            240
        )
    }

    #[test]
    fn sample2() {
        let m = 3;
        let n = 1;
        let introverts_count = 2;
        let extroverts_count = 1;
        assert_eq!(
            Solution::get_max_grid_happiness(m, n, introverts_count, extroverts_count),
            260
        )
    }

    #[test]
    fn sample3() {
        let m = 2;
        let n = 2;
        let introverts_count = 4;
        let extroverts_count = 0;
        assert_eq!(
            Solution::get_max_grid_happiness(m, n, introverts_count, extroverts_count),
            240
        )
    }
}
