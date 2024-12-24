pub struct Solution;

mod seal {
    use std::iter::once;

    struct SubSetIterator1 {
        mask: u32,
        sub: u32,
    }

    impl SubSetIterator1 {
        fn new(mask: u32) -> Self {
            Self { mask, sub: mask }
        }
    }

    impl Iterator for SubSetIterator1 {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.sub == 0 {
                None
            } else {
                let ret = self.sub;
                self.sub = (self.sub - 1) & self.mask;
                Some(ret)
            }
        }
    }

    pub fn subset_iter(mask: u32) -> impl Iterator<Item = u32> {
        SubSetIterator1::new(mask).chain(once(0))
    }
}

use seal::subset_iter;

impl Solution {
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        let m = seats.len();
        let n = seats[0].len();

        let mask_seats: Vec<u32> = seats
            .iter()
            .map(|row| {
                row.iter()
                    .fold(0, |acc, &seat| (acc << 1) | if seat == '#' { 0 } else { 1 })
            })
            .collect();

        let mut dp = vec![vec![0; 1 << n]; m];

        // dp[0]
        for sub in subset_iter(mask_seats[0]) {
            if (sub & (sub << 1)) == 0 && (sub & (sub >> 1)) == 0 {
                dp[0][sub as usize] = sub.count_ones() as i32;
            }
        }

        // dp[i] where i > 0
        for i in 1..m {
            for sub in subset_iter(mask_seats[i]) {
                for prev_sub in subset_iter(mask_seats[i - 1]) {
                    if (sub & (prev_sub << 1)) == 0
                        && (sub & (prev_sub >> 1)) == 0
                        && (sub & (sub << 1)) == 0
                        && (sub & (sub >> 1)) == 0
                    {
                        dp[i][sub as usize] = dp[i][sub as usize]
                            .max(dp[i - 1][prev_sub as usize] + sub.count_ones() as i32);
                    }
                }
            }
        }

        dp[m - 1].iter().copied().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let seats = [
            ["#", ".", "#", "#", ".", "#"],
            [".", "#", "#", "#", "#", "."],
            ["#", ".", "#", "#", ".", "#"],
        ];
        let seats: Vec<Vec<char>> = seats
            .into_iter()
            .map(|row| row.into_iter().map(|s| s.chars().next().unwrap()).collect())
            .collect();
        assert_eq!(Solution::max_students(seats), 4)
    }

    #[test]
    fn sample2() {
        let seats = [[".", "#"], ["#", "#"], ["#", "."], ["#", "#"], [".", "#"]];
        let seats: Vec<Vec<char>> = seats
            .into_iter()
            .map(|row| row.into_iter().map(|s| s.chars().next().unwrap()).collect())
            .collect();
        assert_eq!(Solution::max_students(seats), 3)
    }

    #[test]
    fn sample3() {
        let seats = [
            ["#", ".", ".", ".", "#"],
            [".", "#", ".", "#", "."],
            [".", ".", "#", ".", "."],
            [".", "#", ".", "#", "."],
            ["#", ".", ".", ".", "#"],
        ];
        let seats: Vec<Vec<char>> = seats
            .into_iter()
            .map(|row| row.into_iter().map(|s| s.chars().next().unwrap()).collect())
            .collect();
        assert_eq!(Solution::max_students(seats), 10)
    }
}
