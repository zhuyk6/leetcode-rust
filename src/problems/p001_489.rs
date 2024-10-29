pub struct Solution;

use std::collections::HashSet;

struct Edge<T> {
    _from: usize,
    to: usize,
    info: T,
}

struct Graph<T> {
    link: Vec<Vec<usize>>,
    edges: Vec<Edge<T>>,
}

impl<T: Clone> Graph<T> {
    fn with_size(n: usize) -> Self {
        Graph {
            link: vec![vec![]; n],
            edges: vec![],
        }
    }

    fn add_edge(&mut self, x: usize, y: usize, info: T) {
        self.link[x].push(self.edges.len());
        self.edges.push(Edge {
            _from: x,
            to: y,
            info: info.clone(),
        });
        self.link[y].push(self.edges.len());
        self.edges.push(Edge {
            _from: y,
            to: x,
            info: info.clone(),
        });
    }
}

impl Graph<usize> {
    #[allow(clippy::too_many_arguments)]
    fn dfs(
        &self,
        x: usize,
        from_link: usize,
        timestamp: &mut u32,
        index: &mut [u32],
        low_index: &mut [u32],
        critical: &mut Vec<usize>,
        fake_critical: &mut HashSet<usize>,
    ) {
        *timestamp += 1;
        index[x] = *timestamp;
        low_index[x] = *timestamp;

        for &eid in &self.link[x] {
            if eid ^ 1 != from_link {
                let e = &self.edges[eid];

                if index[e.to] == u32::MAX {
                    self.dfs(
                        e.to,
                        eid,
                        timestamp,
                        index,
                        low_index,
                        critical,
                        fake_critical,
                    );
                }
                low_index[x] = low_index[x].min(low_index[e.to]);

                if index[e.to] == low_index[e.to] && index[x] < index[e.to] {
                    critical.push(e.info);
                } else {
                    fake_critical.insert(e.info);
                }
            }
        }
    }
}

struct JoinSet {
    fa: Vec<usize>,
}

impl JoinSet {
    fn with_size(n: usize) -> Self {
        JoinSet {
            fa: (0..n).collect(),
        }
    }

    fn find_fa(&mut self, x: usize) -> usize {
        if self.fa[x] != x {
            self.fa[x] = self.find_fa(self.fa[x]);
        }
        self.fa[x]
    }
}

impl Solution {
    pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let m = edges.len();

        // sort edges and group by cost
        let groups = {
            let mut edge_sorted_indices: Vec<usize> = (0..m).collect();
            edge_sorted_indices.sort_by_key(|&idx| edges[idx][2]);

            let mut groups = vec![];
            let mut prev_cost = -1;
            let mut cur_group: Option<Vec<usize>> = None;
            for idx in edge_sorted_indices {
                if edges[idx][2] > prev_cost {
                    if let Some(g) = cur_group.take() {
                        groups.push(g);
                    }
                    cur_group = Some(vec![]);
                }
                if let Some(g) = cur_group.as_mut() {
                    g.push(idx)
                }
                prev_cost = edges[idx][2];
            }
            groups.push(cur_group.expect("edges can't be empty"));
            groups
        };

        dbg!(&groups);

        let mut critical = vec![];
        let mut fake_critical = HashSet::new();

        let mut index = vec![u32::MAX; n];
        let mut low_index = vec![u32::MAX; n];

        // build the MST
        let mut joinset = JoinSet::with_size(n);
        for g in groups {
            dbg!(&g);

            // build graph
            let mut graph = Graph::<usize>::with_size(n);
            for &idx in &g {
                let x = edges[idx][0] as usize;
                let y = edges[idx][1] as usize;

                let fx = joinset.find_fa(x);
                let fy = joinset.find_fa(y);
                if fx != fy {
                    // this edge is available
                    graph.add_edge(fx, fy, idx);
                }
            }

            // find critical
            index.fill(u32::MAX);
            low_index.fill(u32::MAX);
            let mut timestamp = 0;

            for x in 0..n {
                if index[x] == u32::MAX {
                    graph.dfs(
                        x,
                        usize::MAX,
                        &mut timestamp,
                        &mut index,
                        &mut low_index,
                        &mut critical,
                        &mut fake_critical,
                    );
                }
            }

            dbg!(&critical);
            dbg!(&fake_critical);

            // add edge
            for &idx in &g {
                let x = edges[idx][0] as usize;
                let y = edges[idx][1] as usize;

                let fx = joinset.find_fa(x);
                let fy = joinset.find_fa(y);
                if fx != fy {
                    joinset.fa[fx] = fy;
                }
            }
        }

        vec![
            critical.into_iter().map(|v| v as i32).collect(),
            fake_critical.into_iter().map(|v| v as i32).collect(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let n = 5;
        let edges = nested_vec![
            [0, 1, 1],
            [1, 2, 1],
            [2, 3, 2],
            [0, 3, 2],
            [0, 4, 3],
            [3, 4, 3],
            [1, 4, 6]
        ];
        let mut left = Solution::find_critical_and_pseudo_critical_edges(n, edges);
        left[0].sort();
        left[1].sort();

        let right = nested_vec![[0, 1], [2, 3, 4, 5]];
        assert_eq!(left, right);
    }

    #[test]
    fn sample2() {
        let n = 4;
        let edges = nested_vec![[0, 1, 1], [1, 2, 1], [2, 3, 1], [0, 3, 1]];
        let mut left = Solution::find_critical_and_pseudo_critical_edges(n, edges);
        left[0].sort();
        left[1].sort();

        let right = nested_vec![[], [0, 1, 2, 3]];
        assert_eq!(left, right);
    }
}
