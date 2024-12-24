pub struct Solution;

struct Dfs {
    digits: Vec<usize>,
    suffix: Vec<usize>,
    max_digit: usize,
    suffix_lower: bool,
    mem: Vec<[Option<u64>; 2]>,
}

impl Dfs {
    fn new(upper_bound: i64, s: &str, max_digit: usize) -> Self {
        let suffix: Vec<_> = s.as_bytes().iter().map(|&c| (c - b'0') as usize).collect();
        let digits = {
            let mut digits = vec![];
            let mut n = upper_bound;
            while n > 0 {
                digits.push((n % 10) as usize);
                n /= 10;
            }
            while digits.len() < suffix.len() {
                digits.push(0);
            }
            digits.reverse();
            digits
        };
        let n = digits.len();
        let m = suffix.len();
        let suffix_lower = !matches!(
            suffix.iter().zip(digits[n - m..].iter()).fold(
                std::cmp::Ordering::Equal,
                |acc, (a, b)| match acc {
                    std::cmp::Ordering::Equal => a.cmp(b),
                    _ => acc,
                },
            ),
            std::cmp::Ordering::Greater
        );
        // dbg!(&suffix, &digits[n - m..], &suffix_lower);
        Self {
            digits,
            suffix,
            max_digit,
            suffix_lower,
            mem: vec![[None; 2]; n],
        }
    }

    fn dfs(&mut self, pos: usize, limit: bool) -> u64 {
        if pos + self.suffix.len() >= self.digits.len() {
            return if limit { self.suffix_lower as u64 } else { 1 };
        }
        if let Some(&ret) = self.mem[pos][limit as usize].as_ref() {
            return ret;
        }
        let mut ret = 0;
        let end = if limit { self.digits[pos] } else { 9 }.min(self.max_digit);
        for i in 0..=end {
            let new_limit = limit && i == self.digits[pos];
            ret += self.dfs(pos + 1, new_limit);
        }
        self.mem[pos][limit as usize] = Some(ret);
        ret
    }
}

impl Solution {
    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
        let mut sol1 = Dfs::new(finish, &s, limit as usize);
        let mut sol2 = Dfs::new(start - 1, &s, limit as usize);
        let tmp1 = sol1.dfs(0, true);
        let tmp2 = sol2.dfs(0, true);
        // dbg!(tmp1, tmp2);
        (tmp1 - tmp2) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let start = 1;
        let finish = 6000;
        let limit = 4;
        let s = "124".to_string();
        assert_eq!(Solution::number_of_powerful_int(start, finish, limit, s), 5);
    }

    #[test]
    fn sample2() {
        let start = 15;
        let finish = 215;
        let limit = 6;
        let s = "10".to_string();
        assert_eq!(Solution::number_of_powerful_int(start, finish, limit, s), 2);
    }

    #[test]
    fn sample3() {
        let start = 1000;
        let finish = 2000;
        let limit = 4;
        let s = "3000".to_string();
        assert_eq!(Solution::number_of_powerful_int(start, finish, limit, s), 0);
    }

    #[test]
    fn issue() {
        let start = 1;
        let finish = 971;
        let limit = 9;
        let s = "17".to_string();
        assert_eq!(
            Solution::number_of_powerful_int(start, finish, limit, s),
            10
        );
    }
}
