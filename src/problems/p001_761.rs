pub struct Solution;

// struct Graph {
//     n: usize,
//     degree: Vec<u32>,
//     adj_mat: Vec<Vec<bool>>,
// }

// impl Graph {
//     fn build(n: usize, edges: Vec<Vec<i32>>) -> Self {
//         let mut degree = vec![0; n];
//         let mut adj_mat = vec![vec![false; n]; n];

//         for e in edges {
//             let x = e[0] as usize - 1;
//             let y = e[1] as usize - 1;
//             degree[x] += 1;
//             degree[y] += 1;
//             adj_mat[x][y] = true;
//             adj_mat[y][x] = true;
//         }
//         Self { n, degree, adj_mat }
//     }

//     fn dfs(&self, x: usize, fx: usize, vis: &mut [bool], ans: &mut u32) {
//         vis[x] = true;
//         for y in 0..self.n {
//             if self.adj_mat[x][y] && y != fx {
//                 if !vis[y] {
//                     self.dfs(y, x, vis, ans);
//                 }
//                 if fx < usize::MAX && self.adj_mat[y][fx] {
//                     let d = self.degree[x] + self.degree[y] + self.degree[fx] - 6;
//                     dbg!(fx, x, y, d);
//                     eprintln!();
//                     if *ans > d {
//                         *ans = d;
//                     }
//                 }
//             }
//         }
//     }

//     fn count_answer(&self) -> u32 {
//         let n = self.degree.len();
//         let mut vis = vec![false; n];
//         let mut ans = u32::MAX;

//         for x in 0..n {
//             if !vis[x] {
//                 self.dfs(x, usize::MAX, &mut vis, &mut ans);
//             }
//         }
//         ans
//     }
// }

impl Solution {
    pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        // let graph = Graph::build(n as usize, edges);
        // match graph.count_answer() {
        //     u32::MAX => -1,
        //     v => v as i32,
        // }

        let n = n as usize;

        use std::collections::HashSet;
        let mut adj_sets = vec![HashSet::new(); n];
        let mut deg = vec![0u32; n];

        for e in &edges {
            let x = e[0] as usize - 1;
            let y = e[1] as usize - 1;
            adj_sets[x].insert(y);
            adj_sets[y].insert(x);
            deg[x] += 1;
            deg[y] += 1;
        }

        let mut ans = u32::MAX;
        for e in &edges {
            let x = e[0] as usize - 1;
            let y = e[1] as usize - 1;

            for &z in adj_sets[x].intersection(&adj_sets[y]) {
                let d = deg[x] + deg[y] + deg[z] - 6;
                ans = ans.min(d);
            }
        }

        match ans {
            u32::MAX => -1,
            v => v as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let n = 6;
        let edges = nested_vec![[1, 2], [1, 3], [3, 2], [4, 1], [5, 2], [3, 6]];
        assert_eq!(Solution::min_trio_degree(n, edges), 3);
    }

    #[test]
    fn sample2() {
        let n = 7;
        let edges = nested_vec![
            [1, 3],
            [4, 1],
            [4, 3],
            [2, 5],
            [5, 6],
            [6, 7],
            [7, 5],
            [2, 6]
        ];
        assert_eq!(Solution::min_trio_degree(n, edges), 0);
    }

    #[test]
    fn issue() {
        let n = 6;
        let edges = nested_vec![
            [6, 5],
            [4, 3],
            [5, 1],
            [1, 4],
            [2, 3],
            [4, 5],
            [2, 6],
            [1, 3]
        ];
        assert_eq!(Solution::min_trio_degree(n, edges), 3);
    }

    #[test]
    fn issue2() {
        let n = 10;
        let edges = nested_vec![
            [3, 4],
            [3, 10],
            [5, 7],
            [9, 5],
            [8, 3],
            [5, 10],
            [9, 8],
            [10, 9],
            [1, 6],
            [1, 3],
            [6, 2],
            [6, 8],
            [4, 8],
            [3, 6],
            [8, 2],
            [9, 1],
            [9, 4],
            [7, 3],
            [7, 6],
            [3, 2],
            [3, 5],
            [5, 2],
            [4, 10],
            [9, 3],
            [5, 8],
            [8, 7],
            [9, 6],
            [10, 1],
            [10, 7],
            [1, 4],
            [2, 9],
            [1, 7]
        ];
        assert_eq!(Solution::min_trio_degree(n, edges), 11);
    }
}
