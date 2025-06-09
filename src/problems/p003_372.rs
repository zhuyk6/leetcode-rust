pub struct Solution;

use std::collections::VecDeque;

struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    fn new(edges: Vec<Vec<i32>>) -> Self {
        let n = edges.len() + 1;
        let mut adj = vec![vec![]; n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj[u].push(v);
            adj[v].push(u); // Assuming undirected graph
        }
        Graph { adj }
    }

    fn bfs(&self, start: usize, k: i32) -> i32 {
        if k < 0 {
            return 0;
        }
        let n = self.adj.len();
        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();

        queue.push_back((start, k));
        visited[start] = true;

        let mut ans = 0;

        while let Some((x, k)) = queue.pop_front() {
            ans += 1;
            if k > 0 {
                for &neighbor in &self.adj[x] {
                    if !visited[neighbor] {
                        visited[neighbor] = true;
                        queue.push_back((neighbor, k - 1));
                    }
                }
            }
        }

        ans
    }
}

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let graph1 = Graph::new(edges1);
        let graph2 = Graph::new(edges2);

        let sum2 = (0..graph2.adj.len())
            .map(|i| graph2.bfs(i, k - 1))
            .max()
            .unwrap_or(0);

        (0..graph1.adj.len())
            .map(|i| graph1.bfs(i, k) + sum2)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let edges1 = nested_vec![[0, 1], [0, 2], [2, 3], [2, 4]];
        let edges2 = nested_vec![[0, 1], [0, 2], [0, 3], [2, 7], [1, 4], [4, 5], [4, 6]];
        let k = 2;
        let ans = vec![9, 7, 9, 8, 8];
        assert_eq!(Solution::max_target_nodes(edges1, edges2, k), ans);
    }

    #[test]
    fn sample2() {
        let edges1 = nested_vec![[0, 1], [0, 2], [0, 3], [0, 4]];
        let edges2 = nested_vec![[0, 1], [1, 2], [2, 3]];
        let k = 1;
        let ans = vec![6, 3, 3, 3, 3];
        assert_eq!(Solution::max_target_nodes(edges1, edges2, k), ans);
    }
}
