pub struct Solution;

use std::collections::HashSet;

fn fact(n: usize) -> i64 {
    let mut ans = 1;
    for i in 2..=n {
        ans *= i as i64;
    }
    ans
}

impl Solution {
    pub fn count_good_integers(n: i32, k: i32) -> i64 {
        let n = n as usize;
        let k = k as i64;

        let mut set: HashSet<[usize; 10]> = HashSet::new();
        fn f(x: i64, n: usize, set: &mut HashSet<[usize; 10]>) -> i64 {
            if x % 10 == 0 {
                return 0;
            }

            let mut cnt = [0; 10];

            let mut x = x;
            while x > 0 {
                let digit = (x % 10) as usize;
                cnt[digit] += 1;
                x /= 10;
            }

            if set.contains(&cnt) {
                return 0;
            }
            set.insert(cnt);

            let mut ans = fact(n);
            for v in cnt {
                ans /= fact(v);
            }

            if cnt[0] > 0 {
                cnt[0] -= 1;
                let mut tmp = fact(n - 1);
                for v in cnt {
                    tmp /= fact(v);
                }
                ans -= tmp;
            }

            ans
        }

        fn calc(x: i64, n: usize, k: i64, set: &mut HashSet<[usize; 10]>) -> i64 {
            let m = n / 2;
            let mut acc = 0;
            let mut y = x;
            for _ in 0..m {
                let digit = y % 10;
                acc = acc * 10 + digit;
                y /= 10;
            }
            if n % 2 == 1 {
                let mut ans = 0;
                for digit in 0..10 {
                    let acc = acc * 10 + digit;
                    let acc = acc * 10i64.pow(n as u32 / 2) + x;
                    if acc % k == 0 {
                        ans += f(acc, n, set);
                    }
                }
                ans
            } else {
                let acc = acc * 10i64.pow(n as u32 / 2) + x;
                if acc % k == 0 { f(acc, n, set) } else { 0 }
            }
        }

        let mut ans = 0;
        for x in 0..10i64.pow(n as u32 / 2) {
            ans += calc(x, n, k, &mut set);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let n = 3;
        let k = 5;
        assert_eq!(Solution::count_good_integers(n, k), 27);
    }

    #[test]
    fn sample2() {
        let n = 1;
        let k = 4;
        assert_eq!(Solution::count_good_integers(n, k), 2);
    }

    #[test]
    fn sample3() {
        let n = 5;
        let k = 6;
        assert_eq!(Solution::count_good_integers(n, k), 2468);
    }

    #[test]
    fn sample4() {
        let n = 5;
        let k = 3;
        assert_eq!(Solution::count_good_integers(n, k), 3573);
    }
}
