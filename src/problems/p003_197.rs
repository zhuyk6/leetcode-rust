pub struct Solution;

struct Solver {
    grid: Vec<Vec<i32>>,
}

impl Solver {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        Solver { grid }
    }

    fn divide1(&self, up_left: (usize, usize), down_right: (usize, usize)) -> i32 {
        assert!(up_left.0 <= down_right.0 && up_left.1 <= down_right.1);

        let mut r0 = usize::MAX;
        let mut c0 = usize::MAX;
        let mut r1 = usize::MIN;
        let mut c1 = usize::MIN;

        for i in up_left.0..=down_right.0 {
            for j in up_left.1..=down_right.1 {
                if self.grid[i][j] == 1 {
                    r0 = r0.min(i);
                    c0 = c0.min(j);
                    r1 = r1.max(i);
                    c1 = c1.max(j);
                }
            }
        }

        if r0 == usize::MAX {
            1
        } else {
            let s = (r1 - r0 + 1) * (c1 - c0 + 1);
            s as i32
        }
    }

    fn divide2(&self, up_left: (usize, usize), down_right: (usize, usize)) -> i32 {
        assert!(up_left.0 <= down_right.0 && up_left.1 <= down_right.1);

        if (down_right.0 - up_left.0 + 1) * (down_right.1 - up_left.1 + 1) < 2 {
            return i32::MAX;
        }

        let mut ret = i32::MAX;

        for r in up_left.0..down_right.0 {
            let tmp = self.divide1(up_left, (r, down_right.1))
                + self.divide1((r + 1, up_left.1), down_right);
            ret = ret.min(tmp);
        }

        for c in up_left.1..down_right.1 {
            let tmp = self.divide1(up_left, (down_right.0, c))
                + self.divide1((up_left.0, c + 1), down_right);
            ret = ret.min(tmp);
        }

        ret
    }

    fn divide3(&self) -> i32 {
        let up_left = (0, 0);
        let down_right = (self.grid.len() - 1, self.grid[0].len() - 1);

        assert!((down_right.0 - up_left.0 + 1) * (down_right.1 - up_left.1 + 1) >= 3);

        let mut ret = i32::MAX;

        for r in up_left.0..down_right.0 {
            let s1 = self.divide1(up_left, (r, down_right.1));
            let s2 = self.divide2((r + 1, up_left.1), down_right);
            ret = ret.min(s1.saturating_add(s2));

            let s2 = self.divide2(up_left, (r, down_right.1));
            let s1 = self.divide1((r + 1, up_left.1), down_right);
            ret = ret.min(s1.saturating_add(s2));
        }

        for c in up_left.1..down_right.1 {
            let s1 = self.divide1(up_left, (down_right.0, c));
            let s2 = self.divide2((up_left.0, c + 1), down_right);
            ret = ret.min(s1.saturating_add(s2));

            let s2 = self.divide2(up_left, (down_right.0, c));
            let s1 = self.divide1((up_left.0, c + 1), down_right);
            ret = ret.min(s1.saturating_add(s2));
        }

        ret
    }
}

impl Solution {
    pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
        let sol = Solver::new(grid);

        sol.divide3()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let grid = nested_vec![[1, 0, 1], [1, 1, 1]];
        assert_eq!(Solution::minimum_sum(grid), 5);
    }

    #[test]
    fn sample2() {
        let grid = nested_vec![[1, 0, 1, 0], [0, 1, 0, 1]];
        assert_eq!(Solution::minimum_sum(grid), 5);
    }

    #[test]
    fn sample3() {
        let grid = nested_vec![[0], [1], [1]];
        assert_eq!(Solution::minimum_sum(grid), 3);
    }
}
