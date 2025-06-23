pub struct Solution;

fn check(x: i64, k: i64) -> bool {
    if x % 10 == 0 {
        return false;
    }
    let mut x = x;
    let mut v: Vec<i64> = vec![];
    while x > 0 {
        v.push(x % k);
        x /= k;
    }
    v.iter().eq(v.iter().rev())
}

fn get_rev(x: i64) -> i64 {
    let mut x = x;
    let mut v: Vec<i64> = vec![];
    while x > 0 {
        v.push(x % 10);
        x /= 10;
    }
    v.iter().fold(0, |acc, &x| acc * 10 + x)
}

struct IterWithEvenLen {
    half: u32,
    k: i64,
    current: i64,
}

impl IterWithEvenLen {
    fn new(half: u32, k: i64) -> Self {
        let current = if half > 0 { 10_i64.pow(half - 1) } else { 1 };

        Self { half, k, current }
    }
}

impl Iterator for IterWithEvenLen {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let max_num = 10_i64.pow(self.half);
        if self.current >= max_num {
            return None;
        }
        for x in self.current..max_num {
            let num = x * 10_i64.pow(self.half) + get_rev(x);
            if check(num, self.k) {
                self.current = x + 1;
                return Some(num);
            }
        }
        self.current = max_num;
        None
    }
}

struct IterWithOddLen {
    half: u32,
    k: i64,
    current: i64,
    middle: i64,
}

impl IterWithOddLen {
    fn new(half: u32, k: i64) -> Self {
        let current = if half > 0 { 10_i64.pow(half - 1) } else { 0 };
        let middle = 0;

        Self {
            half,
            k,
            current,
            middle,
        }
    }
}

impl Iterator for IterWithOddLen {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let max_num = 10_i64.pow(self.half);
        if self.current >= max_num {
            return None;
        }
        for x in self.current..max_num {
            for y in self.middle..10 {
                let num = x * 10_i64.pow(self.half + 1) + y * 10_i64.pow(self.half) + get_rev(x);
                if check(num, self.k) {
                    self.current = x;
                    self.middle = y + 1;
                    return Some(num);
                }
            }
            self.middle = 0;
        }
        self.current = max_num;
        None
    }
}

#[allow(unused)]
struct IterWithLen {
    len: u32,
    k: i64,
    iter: Box<dyn Iterator<Item = i64>>,
}

impl IterWithLen {
    fn new(len: u32, k: i64) -> Self {
        let half = len / 2;
        if len % 2 == 0 {
            Self {
                len,
                k,
                iter: Box::new(IterWithEvenLen::new(half, k)),
            }
        } else {
            Self {
                len,
                k,
                iter: Box::new(IterWithOddLen::new(half, k)),
            }
        }
    }
}

impl Iterator for IterWithLen {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

struct Iter {
    k: i64,
    len: u32,
    iter: Box<dyn Iterator<Item = i64>>,
}

impl Iter {
    fn new(k: i64) -> Self {
        Self {
            k,
            len: 1,
            iter: Box::new(IterWithLen::new(1, k)),
        }
    }
}

impl Iterator for Iter {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(num) = self.iter.next() {
            return Some(num);
        }
        self.len += 1;
        self.iter = Box::new(IterWithLen::new(self.len, self.k));
        self.next()
    }
}

impl Solution {
    pub fn k_mirror(k: i32, n: i32) -> i64 {
        Iter::new(k as i64).take(n as usize).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rev() {
        let x = 12345;
        assert_eq!(get_rev(x), 54321);
    }

    #[test]
    fn sample1() {
        let k = 2;
        let n = 5;
        let expected = 25;
        assert_eq!(Solution::k_mirror(k, n), expected);
    }

    #[test]
    fn sample2() {
        let k = 3;
        let n = 7;
        let expected = 499;
        assert_eq!(Solution::k_mirror(k, n), expected);
    }

    #[test]
    fn sample3() {
        let k = 7;
        let n = 17;
        let expected = 20379000;
        assert_eq!(Solution::k_mirror(k, n), expected);
    }
}
