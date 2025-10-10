pub struct Solution;

use std::cmp::Reverse;

impl Solution {
    pub fn number_of_pairs(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|p| (p[0], Reverse(p[1])));

        let mut ans = 0;
        let n = points.len();

        for i in 0..n {
            let mut min_y = i32::MAX;
            for j in (0..i).rev() {
                if points[j][1] < min_y && points[j][1] >= points[i][1] {
                    min_y = points[j][1];
                    ans += 1;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let points = nested_vec![[1, 1], [2, 2], [3, 3]];
        assert_eq!(Solution::number_of_pairs(points), 0);
    }

    #[test]
    fn sample2() {
        let points = nested_vec![[6, 2], [4, 4], [2, 6]];
        assert_eq!(Solution::number_of_pairs(points), 2);
    }

    #[test]
    fn sample3() {
        let points = nested_vec![[3, 1], [1, 3], [1, 1]];
        assert_eq!(Solution::number_of_pairs(points), 2);
    }

    #[test]
    fn issue() {
        let points = nested_vec![[3, 2], [6, 0], [3, 4], [1, 0]];
        assert_eq!(Solution::number_of_pairs(points), 3);
    }
}
