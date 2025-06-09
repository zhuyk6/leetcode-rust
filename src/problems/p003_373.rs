pub struct Solution;

// n <= 10^5
#[derive(Debug, Clone)]
struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    fn new(edges: Vec<Vec<i32>>) -> Self {
        let n = edges.len() + 1;
        let mut adj = vec![vec![]; n];
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            adj[u].push(v);
            adj[v].push(u);
        }
        Graph { adj }
    }

    fn get_odd_even_count(&self) -> (u32, u32, Vec<bool>) {
        let mut odd_count = 0;
        let mut even_count = 0;

        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        let mut vis = vec![false; self.adj.len()];
        let mut layer = vec![false; self.adj.len()];

        queue.push_back((0, false));
        vis[0] = true;
        layer[0] = false;

        while let Some((x, is_odd)) = queue.pop_front() {
            if is_odd {
                odd_count += 1;
            } else {
                even_count += 1;
            }
            layer[x] = is_odd;

            for &y in &self.adj[x] {
                if !vis[y] {
                    vis[y] = true;
                    queue.push_back((y, !is_odd));
                }
            }
        }

        (even_count, odd_count, layer)
    }
}

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        let graph1 = Graph::new(edges1);
        let graph2 = Graph::new(edges2);

        let (num_even_graph2, num_odd_graph2, _) = graph2.get_odd_even_count();
        let num2 = num_odd_graph2.max(num_even_graph2);

        let (num_even_graph1, num_odd_graph1, layer1) = graph1.get_odd_even_count();

        layer1
            .into_iter()
            .map(|is_odd| {
                let ans = if is_odd {
                    num_odd_graph1 + num2
                } else {
                    num_even_graph1 + num2
                };
                ans as i32
            })
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
        let ans = vec![8, 7, 7, 8, 8];
        assert_eq!(Solution::max_target_nodes(edges1, edges2), ans);
    }

    #[test]
    fn sample2() {
        let edges1 = nested_vec![[0, 1], [0, 2], [0, 3], [0, 4]];
        let edges2 = nested_vec![[0, 1], [1, 2], [2, 3]];
        let ans = vec![3, 6, 6, 6, 6];
        assert_eq!(Solution::max_target_nodes(edges1, edges2), ans);
    }
}
