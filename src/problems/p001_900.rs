pub struct Solution;

impl Solution {
    pub fn earliest_and_latest(n: i32, first_player: i32, second_player: i32) -> Vec<i32> {
        use std::collections::HashMap;

        fn dfs(
            n: usize,
            first: usize,
            second: usize,
            mem: &mut HashMap<(usize, usize, usize), (i32, i32)>,
        ) -> (i32, i32) {
            debug_assert!(first < n && second < n);
            debug_assert!(first != second);

            if first + second == n - 1 {
                return (1, 1);
            }

            if let Some(&ret) = mem.get(&(n, first, second)) {
                return ret;
            }

            let mut earliest = i32::MAX;
            let mut latest = i32::MIN;

            let new_n = if n % 2 == 0 { n / 2 } else { n / 2 + 1 };
            let len = n / 2;
            for mask in 0..(1 << len) {
                let mut new_first = 0;
                let mut new_second = 0;
                if first < second {
                    new_second += 1;
                } else {
                    new_first += 1;
                }
                if n % 2 == 1 && first != n / 2 && second != n / 2 {
                    if first > n / 2 {
                        new_first += 1;
                    }
                    if second > n / 2 {
                        new_second += 1;
                    }
                }

                for p in 0..len {
                    let q = n - 1 - p;
                    if p == first || p == second || q == first || q == second {
                        continue;
                    }

                    // first
                    if p < first && mask & (1 << p) != 0 {
                        new_first += 1;
                    }
                    if q < first && mask & (1 << p) == 0 {
                        new_first += 1;
                    }

                    // second
                    if p < second && mask & (1 << p) != 0 {
                        new_second += 1;
                    }
                    if q < second && mask & (1 << p) == 0 {
                        new_second += 1;
                    }
                }

                let tmp = dfs(new_n, new_first, new_second, mem);
                earliest = earliest.min(tmp.0 + 1);
                latest = latest.max(tmp.1 + 1);
            }

            mem.insert((n, first, second), (earliest, latest));
            (earliest, latest)
        }

        let n = n as usize;
        let first = first_player as usize - 1;
        let second = second_player as usize - 1;
        let mut mem = HashMap::new();
        let (earliest, latest) = dfs(n, first, second, &mut mem);
        vec![earliest, latest]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let n = 11;
        let first_player = 2;
        let second_player = 4;
        assert_eq!(
            Solution::earliest_and_latest(n, first_player, second_player),
            vec![3, 4]
        );
    }

    #[test]
    fn sample2() {
        let n = 5;
        let first_player = 1;
        let second_player = 5;
        assert_eq!(
            Solution::earliest_and_latest(n, first_player, second_player),
            vec![1, 1]
        );
    }

    #[test]
    fn sample3() {
        let n = 3;
        let first_player = 2;
        let second_player = 3;
        assert_eq!(
            Solution::earliest_and_latest(n, first_player, second_player),
            vec![2, 2]
        );
    }
}
