pub struct Solution;

struct Graph {
    to: Vec<Vec<(usize, i32)>>,
}

impl Graph {
    fn with_nodes(n: usize) -> Self {
        Self {
            to: vec![vec![]; n],
        }
    }

    #[inline]
    fn add_edge(&mut self, x: usize, y: usize, w: i32) {
        self.to[x].push((y, w));
    }

    fn dijkstra(&self, s: usize) -> Vec<i64> {
        let n = self.to.len();
        let mut dis = vec![i64::MAX; n];

        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut heap: BinaryHeap<(Reverse<i64>, usize)> = BinaryHeap::with_capacity(n);

        dis[s] = 0;
        heap.push((Reverse(0), s));
        while let Some((Reverse(dx), x)) = heap.pop() {
            if dx > dis[x] {
                continue;
            }
            for &(y, w) in &self.to[x] {
                let dy = dx + w as i64;
                if dis[y] > dy {
                    dis[y] = dy;
                    heap.push((Reverse(dy), y));
                }
            }
        }

        dis
    }
}

impl Solution {
    pub fn minimum_weight(n: i32, edges: Vec<Vec<i32>>, src1: i32, src2: i32, dest: i32) -> i64 {
        let mut graph = Graph::with_nodes(n as usize);
        let mut graph_rev = Graph::with_nodes(n as usize);
        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            let w = e[2];
            graph.add_edge(x, y, w);
            graph_rev.add_edge(y, x, w);
        }

        let dist_from_s1 = graph.dijkstra(src1 as usize);
        let dist_from_s2 = graph.dijkstra(src2 as usize);
        let dist_from_t = graph_rev.dijkstra(dest as usize);

        match (0..n as usize)
            .map(|x| {
                dist_from_s1[x]
                    .saturating_add(dist_from_s2[x])
                    .saturating_add(dist_from_t[x])
            })
            .min()
            .unwrap()
        {
            i64::MAX => -1,
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
        let n = 6;
        let edges = nested_vec![
            [0, 2, 2],
            [0, 5, 6],
            [1, 0, 3],
            [1, 4, 5],
            [2, 1, 1],
            [2, 3, 3],
            [2, 3, 4],
            [3, 4, 2],
            [4, 5, 1]
        ];
        let src1 = 0;
        let src2 = 1;
        let dest = 5;
        assert_eq!(Solution::minimum_weight(n, edges, src1, src2, dest), 9);
    }

    #[test]
    fn sample2() {
        let n = 3;
        let edges = nested_vec![[0, 1, 1], [2, 1, 1]];
        let src1 = 0;
        let src2 = 1;
        let dest = 2;
        assert_eq!(Solution::minimum_weight(n, edges, src1, src2, dest), -1);
    }
}
