pub struct Solution;

struct Graph {
    graph: Vec<Vec<usize>>,
}

struct SccState {
    index: u32,
    pre: Vec<u32>,
    low: Vec<u32>,
    stack: Vec<usize>,
    scc: Vec<Vec<usize>>,
    belong: Vec<usize>,
}

impl Graph {
    fn new(graph: Vec<Vec<usize>>) -> Self {
        Self { graph }
    }

    fn dfs(&self, s: &mut SccState, x: usize) {
        s.stack.push(x);
        s.index += 1;
        s.pre[x] = s.index;
        s.low[x] = s.index;

        for &y in &self.graph[x] {
            if s.pre[y] == u32::MAX {
                self.dfs(s, y);
                s.low[x] = s.low[x].min(s.low[y]);
            } else if s.belong[y] == usize::MAX {
                s.low[x] = s.low[x].min(s.low[y]);
            }
        }

        if s.pre[x] == s.low[x] {
            let scc_id = s.scc.len();
            let mut v = vec![];
            while let Some(y) = s.stack.pop() {
                s.belong[y] = scc_id;
                v.push(y);
                if y == x {
                    s.scc.push(v);
                    break;
                }
            }
        }
    }

    fn build(&self) -> SccState {
        let n = self.graph.len();
        let mut s = SccState {
            index: 0,
            pre: vec![u32::MAX; n],
            low: vec![u32::MAX; n],
            stack: vec![],
            scc: vec![],
            belong: vec![usize::MAX; n],
        };

        for x in 0..n {
            if s.pre[x] == u32::MAX {
                self.dfs(&mut s, x);
            }
        }

        s
    }
}

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let n = edges.len();

        let mut graph = vec![vec![]; n];
        for (u, v) in edges.into_iter().enumerate() {
            if v != -1 {
                graph[u].push(v as usize);
            }
        }

        let graph = Graph::new(graph);
        let scc = graph.build();

        match scc.scc.iter().map(|g| g.len()).max() {
            Some(ans) if ans > 1 => ans as i32,
            _ => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let edges = vec![3, 3, 4, 2, 3];
        assert_eq!(Solution::longest_cycle(edges), 3);
    }

    #[test]
    fn sample2() {
        let edges = vec![2, -1, 3, 1];
        assert_eq!(Solution::longest_cycle(edges), -1);
    }
}
