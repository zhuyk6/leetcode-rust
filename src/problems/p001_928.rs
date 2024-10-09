struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_cost(max_time: i32, edges: Vec<Vec<i32>>, passing_fees: Vec<i32>) -> i32 {
        let n = passing_fees.len();
        let mut graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            let c = e[2] as usize;
            graph[x].push((y, c));
            graph[y].push((x, c));
        }

        let max_time = max_time as usize;
        let mut f = vec![vec![i32::MAX; n]; max_time + 1];

        f[0][0] = passing_fees[0];

        for t in 0..max_time {
            #[allow(clippy::needless_range_loop)]
            for x in 0..n {
                if f[t][x] == i32::MAX {
                    continue;
                }
                for &(y, c) in &graph[x] {
                    if t + c <= max_time {
                        f[t + c][y] = f[t + c][y].min(f[t][x] + passing_fees[y]);
                    }
                }
            }
        }

        let ans = (0..=max_time).map(|t| f[t][n - 1]).min().unwrap();
        match ans {
            i32::MAX => -1,
            v => v,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let max_time = 30;
        let edges = nested_vec![
            [0, 1, 10],
            [1, 2, 10],
            [2, 5, 10],
            [0, 3, 1],
            [3, 4, 10],
            [4, 5, 15]
        ];
        let passing_fees = vec![5, 1, 2, 20, 20, 3];

        assert_eq!(Solution::min_cost(max_time, edges, passing_fees), 11);
    }

    #[test]
    fn sample2() {
        let max_time = 29;
        let edges = nested_vec![
            [0, 1, 10],
            [1, 2, 10],
            [2, 5, 10],
            [0, 3, 1],
            [3, 4, 10],
            [4, 5, 15]
        ];
        let passing_fees = vec![5, 1, 2, 20, 20, 3];

        assert_eq!(Solution::min_cost(max_time, edges, passing_fees), 48);
    }

    #[test]
    fn sample3() {
        let max_time = 25;
        let edges = nested_vec![
            [0, 1, 10],
            [1, 2, 10],
            [2, 5, 10],
            [0, 3, 1],
            [3, 4, 10],
            [4, 5, 15]
        ];
        let passing_fees = vec![5, 1, 2, 20, 20, 3];

        assert_eq!(Solution::min_cost(max_time, edges, passing_fees), -1);
    }
}
