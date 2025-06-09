pub struct Solution;

const M: usize = 26;

struct Graph {
    adj: Vec<Vec<usize>>,
    colors: Vec<usize>,
}

impl Graph {
    fn new(colors: String, edges: Vec<Vec<i32>>) -> Self {
        let n = colors.len();
        let mut adj = vec![vec![]; n];
        let mut color_vec = vec![0; n];

        for (i, c) in colors.chars().enumerate() {
            color_vec[i] = (c as u8 - b'a') as usize;
        }

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj[u].push(v);
        }

        Graph {
            adj,
            colors: color_vec,
        }
    }
}

struct Visitor {
    count: Vec<Option<[u32; M]>>,
}

impl Visitor {
    fn on_enter(&mut self, _graph: &Graph, x: usize) {
        self.count[x] = Some([0; M]);
    }

    fn on_tree_edge(&mut self, _graph: &Graph, x: usize, y: usize) {
        if let Some(c) = self.count[y] {
            if let Some(count) = self.count[x].as_mut() {
                for i in 0..M {
                    count[i] = count[i].max(c[i]);
                }
            }
        } else {
            self.count[x] = None;
        }
    }

    fn on_back_edge(&mut self, _graph: &Graph, x: usize, _y: usize) {
        self.count[x] = None;
    }

    fn on_cross_edge(&mut self, _graph: &Graph, x: usize, y: usize) {
        if let Some(c) = self.count[y] {
            if let Some(count) = self.count[x].as_mut() {
                for i in 0..M {
                    count[i] = count[i].max(c[i]);
                }
            }
        } else {
            self.count[x] = None;
        }
    }

    fn on_leave(&mut self, graph: &Graph, x: usize) {
        if let Some(count) = self.count[x].as_mut() {
            count[graph.colors[x]] += 1;
        }
    }
}

fn dfs_traverse(graph: &Graph) -> Option<u32> {
    fn dfs(
        graph: &Graph,
        x: usize,
        vis: &mut [bool],
        stack: &mut Vec<usize>,
        in_stack: &mut [bool],
        visitor: &mut Visitor,
    ) {
        vis[x] = true;
        stack.push(x);
        in_stack[x] = true;
        visitor.on_enter(graph, x);

        for &y in &graph.adj[x] {
            if !vis[y] {
                dfs(graph, y, vis, stack, in_stack, visitor);
                visitor.on_tree_edge(graph, x, y);
            } else if in_stack[y] {
                visitor.on_back_edge(graph, x, y);
            } else {
                visitor.on_cross_edge(graph, x, y);
            }
        }

        visitor.on_leave(graph, x);
        stack.pop();
        in_stack[x] = false;
    }

    let n = graph.adj.len();
    let mut visitor = Visitor {
        count: vec![None; n],
    };

    let mut vis = vec![false; n];
    let mut stack = Vec::new();
    let mut in_stack = vec![false; n];

    for i in 0..n {
        if !vis[i] {
            dfs(graph, i, &mut vis, &mut stack, &mut in_stack, &mut visitor);
        }
    }

    let mut max_value = 0;
    for count in visitor.count {
        if let Some(c) = count {
            let max_color_value = c.iter().max().unwrap_or(&0);
            max_value = max_value.max(*max_color_value);
        } else {
            return None; // Cycle detected
        }
    }
    Some(max_value)
}

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let graph = Graph::new(colors, edges);
        match dfs_traverse(&graph) {
            Some(max_value) => max_value as i32,
            None => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let colors = "abaca".to_string();
        let edges = nested_vec![[0, 1], [0, 2], [2, 3], [3, 4]];
        assert_eq!(Solution::largest_path_value(colors, edges), 3);
    }

    #[test]
    fn sample2() {
        let colors = "a".to_string();
        let edges = nested_vec![[0, 0]];
        assert_eq!(Solution::largest_path_value(colors, edges), -1);
    }

    #[test]
    fn issue1() {
        let colors = "hhqhuqhqff".to_string();
        let edges = nested_vec![
            [0, 1],
            [0, 2],
            [2, 3],
            [3, 4],
            [3, 5],
            [5, 6],
            [2, 7],
            [6, 7],
            [7, 8],
            [3, 8],
            [5, 8],
            [8, 9],
            [3, 9],
            [6, 9]
        ];
        assert_eq!(Solution::largest_path_value(colors, edges), 3);
    }
}
