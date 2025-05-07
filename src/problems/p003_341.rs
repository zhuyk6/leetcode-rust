pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let m = move_time.len();
        let n = move_time[0].len();

        let mut dis: Vec<Vec<i32>> = vec![vec![i32::MAX; n]; m];
        let mut vis: Vec<Vec<bool>> = vec![vec![false; n]; m];
        let mut heap: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();

        dis[0][0] = 0;
        heap.push((Reverse(0), 0, 0));

        while let Some((Reverse(d), x, y)) = heap.pop() {
            if vis[x][y] {
                continue;
            }
            vis[x][y] = true;

            if (x, y) == (m - 1, n - 1) {
                break;
            }

            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let xx = (x as i32 + dx) as usize;
                let yy = (y as i32 + dy) as usize;
                if xx < m && yy < n && !vis[xx][yy] {
                    let dd = move_time[xx][yy].max(d) + 1;
                    if dd < dis[xx][yy] {
                        dis[xx][yy] = dd;
                        heap.push((Reverse(dd), xx, yy));
                    }
                }
            }
        }
        dis[m - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn sample1() {
        let move_time = array![[0, 4], [4, 4]];
        let move_time: Vec<Vec<i32>> = move_time
            .axis_iter(ndarray::Axis(0))
            .map(|row| row.to_vec())
            .collect();
        assert_eq!(Solution::min_time_to_reach(move_time), 6);
    }

    #[test]
    fn sample2() {
        let move_time = array![[0, 0, 0], [0, 0, 0]];
        let move_time: Vec<Vec<i32>> = move_time
            .axis_iter(ndarray::Axis(0))
            .map(|row| row.to_vec())
            .collect();
        assert_eq!(Solution::min_time_to_reach(move_time), 3);
    }
}
