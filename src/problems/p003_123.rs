pub struct Solution;

struct Graph {
    to: Vec<Vec<(usize, i32)>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Self {
            to: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, x: usize, y: usize, w: i32) {
        self.to[x].push((y, w));
    }

    fn disjkstra(&self, start: usize) -> Vec<i64> {
        let n = self.to.len();
        let mut dist = vec![i64::MAX; n];
        use std::cmp::Reverse;
        let mut heap = std::collections::BinaryHeap::new();
        dist[start] = 0;
        heap.push(Reverse((0, start)));
        while let Some(Reverse((dx, x))) = heap.pop() {
            if dx > dist[x] {
                continue;
            }
            for &(y, w) in &self.to[x] {
                let dy = dx + w as i64;
                if let Some(d) = dist.get_mut(y)
                    && *d > dy
                {
                    *d = dy;
                    heap.push(Reverse((dy, y)));
                }
            }
        }
        dist
    }
}

impl Solution {
    pub fn find_answer(n: i32, edges: Vec<Vec<i32>>) -> Vec<bool> {
        let n = n as usize;
        let mut graph = Graph::new(n);
        for e in &edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            let w = e[2];
            graph.add_edge(x, y, w);
            graph.add_edge(y, x, w);
        }

        let dist_start = graph.disjkstra(0);
        let dist_end = graph.disjkstra(n - 1);

        let shortest = dist_start[n - 1];
        edges
            .into_iter()
            .map(|e| {
                let x = e[0] as usize;
                let y = e[1] as usize;
                let w = e[2];

                let calc = |x: usize, y: usize| -> bool {
                    match (dist_start[x], dist_end[y]) {
                        (i64::MAX, _) | (_, i64::MAX) => false,
                        (dx, dy) => {
                            let d = dx + w as i64 + dy;
                            d == shortest
                        }
                    }
                };

                calc(x, y) || calc(y, x)
            })
            .collect()
    }
}
