pub struct Solution;

struct Graph {
    to: Vec<Vec<(usize, u32)>>,
}

impl Graph {
    fn with_nodes(n: usize) -> Self {
        Self {
            to: vec![vec![]; n],
        }
    }

    #[inline(always)]
    fn add_edge(&mut self, x: usize, y: usize, w: u32) {
        self.to[x].push((y, w));
    }

    fn dijkstra(&self, s: usize) -> Vec<u32> {
        let n = self.to.len();
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut heap: BinaryHeap<(Reverse<u32>, usize)> = BinaryHeap::with_capacity(n);
        let mut dis = vec![u32::MAX; n];
        let mut vis = vec![false; n];

        dis[s] = 0;
        heap.push((Reverse(0), s));
        while let Some((Reverse(dx), x)) = heap.pop() {
            if !vis[x] {
                vis[x] = true;
                for &(y, w) in &self.to[x] {
                    let dy = dx + w;
                    if dis[y] > dy {
                        dis[y] = dy;
                        heap.push((Reverse(dy), y));
                    }
                }
            }
        }
        dis
    }
}

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let index = |r: usize, c: usize| -> usize { r * n + c };

        let mut graph = Graph::with_nodes(m * n);

        #[allow(clippy::needless_range_loop)]
        for i in 0..m {
            for j in 0..n {
                let x = index(i, j);
                match grid[i][j] {
                    1 => {
                        if j + 1 < n {
                            graph.add_edge(x, index(i, j + 1), 0);
                        }
                        if j > 0 {
                            graph.add_edge(x, index(i, j - 1), 1);
                        }
                        if i + 1 < m {
                            graph.add_edge(x, index(i + 1, j), 1);
                        }
                        if i > 0 {
                            graph.add_edge(x, index(i - 1, j), 1);
                        }
                    }
                    2 => {
                        if j + 1 < n {
                            graph.add_edge(x, index(i, j + 1), 1);
                        }
                        if j > 0 {
                            graph.add_edge(x, index(i, j - 1), 0);
                        }
                        if i + 1 < m {
                            graph.add_edge(x, index(i + 1, j), 1);
                        }
                        if i > 0 {
                            graph.add_edge(x, index(i - 1, j), 1);
                        }
                    }
                    3 => {
                        if j + 1 < n {
                            graph.add_edge(x, index(i, j + 1), 1);
                        }
                        if j > 0 {
                            graph.add_edge(x, index(i, j - 1), 1);
                        }
                        if i + 1 < m {
                            graph.add_edge(x, index(i + 1, j), 0);
                        }
                        if i > 0 {
                            graph.add_edge(x, index(i - 1, j), 1);
                        }
                    }
                    4 => {
                        if j + 1 < n {
                            graph.add_edge(x, index(i, j + 1), 1);
                        }
                        if j > 0 {
                            graph.add_edge(x, index(i, j - 1), 1);
                        }
                        if i + 1 < m {
                            graph.add_edge(x, index(i + 1, j), 1);
                        }
                        if i > 0 {
                            graph.add_edge(x, index(i - 1, j), 0);
                        }
                    }
                    _ => panic!("Invalid input!"),
                }
            }
        }

        let dis = graph.dijkstra(0);
        dis[m * n - 1] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let grid = nested_vec![[1, 1, 1, 1], [2, 2, 2, 2], [1, 1, 1, 1], [2, 2, 2, 2]];
        assert_eq!(Solution::min_cost(grid), 3);
    }

    #[test]
    fn sample2() {
        let grid = nested_vec![[1, 1, 3], [3, 2, 2], [1, 1, 4]];
        assert_eq!(Solution::min_cost(grid), 0);
    }

    #[test]
    fn sample3() {
        let grid = nested_vec![[1, 2], [4, 3]];
        assert_eq!(Solution::min_cost(grid), 1);
    }

    #[test]
    fn sample4() {
        let grid = nested_vec![[2, 2, 2], [2, 2, 2]];
        assert_eq!(Solution::min_cost(grid), 3);
    }

    #[test]
    fn sample5() {
        let grid = nested_vec![[4]];
        assert_eq!(Solution::min_cost(grid), 0);
    }
}
