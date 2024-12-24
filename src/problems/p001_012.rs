pub struct Solution;

struct Dfs {
    digits: Vec<usize>,
    mem: Vec<[[[Option<u32>; 2]; 2]; 1024]>,
}

impl Dfs {
    fn new(digits: Vec<usize>) -> Self {
        let m = digits.len();
        Self {
            digits,
            mem: vec![[[[None; 2]; 2]; 1024]; m],
        }
    }

    fn dfs(&mut self, pos: usize, limit: bool, leading_zero: bool, used_mask: usize) -> u32 {
        if pos == self.digits.len() {
            return 1;
        }
        if let Some(&res) = self.mem[pos][used_mask][limit as usize][leading_zero as usize].as_ref()
        {
            return res;
        }

        let mut res = 0;
        let end = if limit { self.digits[pos] } else { 9 };
        for i in 0..=end {
            if (used_mask >> i) & 1 == 1 {
                continue;
            }
            res += self.dfs(
                pos + 1,
                limit && i == end,
                leading_zero && i == 0,
                if leading_zero && i == 0 {
                    0
                } else {
                    used_mask | (1 << i)
                },
            );
        }
        self.mem[pos][used_mask][limit as usize][leading_zero as usize] = Some(res);
        res
    }
}

impl Solution {
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        let digits = {
            let mut n = n;
            let mut digits = Vec::new();
            while n > 0 {
                digits.push((n % 10) as usize);
                n /= 10;
            }
            digits.reverse();
            digits
        };

        // dbg!(&digits);

        let mut sol = Dfs::new(digits);
        let tmp = sol.dfs(0, true, true, 0);
        // dbg!(&tmp);
        n - tmp as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let n = 20;
        assert_eq!(Solution::num_dup_digits_at_most_n(n), 1)
    }

    #[test]
    fn sample2() {
        let n = 100;
        assert_eq!(Solution::num_dup_digits_at_most_n(n), 10)
    }

    #[test]
    fn sample3() {
        let n = 1000;
        assert_eq!(Solution::num_dup_digits_at_most_n(n), 262)
    }

    #[test]
    fn sample4() {
        let n = 123456789;
        assert_eq!(Solution::num_dup_digits_at_most_n(n), 121064705)
    }
}
