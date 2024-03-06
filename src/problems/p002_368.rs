struct Solution;

#[allow(clippy::upper_case_acronyms)]
struct DFS<'a> {
    graph: &'a Vec<Vec<usize>>,
    cant_visit: Vec<bool>,
}

#[allow(unused)]
impl<'a> DFS<'a> {
    fn new(graph: &'a Vec<Vec<usize>>, restricted: Vec<i32>) -> Self {
        let n = graph.len();
        let mut cant_visit = vec![false; n];
        for x in restricted {
            cant_visit[x as usize] = true;
        }
        DFS { graph, cant_visit }
    }

    fn dfs(&self, x: usize, fa: usize) -> i32 {
        let mut ans = 1;
        for &y in &self.graph[x] {
            if y != fa && !self.cant_visit[y] {
                ans += self.dfs(y, x);
            }
        }
        ans
    }
}

#[allow(unused)]
impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let n = n as usize;

        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            graph[x].push(y);
            graph[y].push(x);
        }

        let dfs = DFS::new(&graph, restricted);
        dfs.dfs(0, n)
    }
}

#[test]
fn test1() {
    let n = 7;
    let edges = [[0, 1], [1, 2], [3, 1], [4, 0], [0, 5], [5, 6]];
    let restricted = vec![4, 5];

    let edges: Vec<Vec<i32>> = edges.into_iter().map(Vec::from).collect();

    assert_eq!(Solution::reachable_nodes(n, edges, restricted), 4);
}

#[test]
fn test2() {
    let n = 7;
    let edges = [[0, 1], [0, 2], [0, 5], [0, 4], [3, 2], [6, 5]];
    let restricted = vec![4, 2, 1];

    let edges: Vec<Vec<i32>> = edges.into_iter().map(Vec::from).collect();

    assert_eq!(Solution::reachable_nodes(n, edges, restricted), 3);
}
