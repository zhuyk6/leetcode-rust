struct Solution;

type Value = [i32; 26];
type Edge = (usize, usize);

struct JointSet {
    parent: Vec<usize>,
}

impl JointSet {
    fn new(n: usize) -> Self {
        JointSet {
            parent: (0..n).collect(),
        }
    }

    fn find_parent(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find_parent(self.parent[x]);
        }
        self.parent[x]
    }

    // x -> y
    fn join(&mut self, x: usize, y: usize) {
        let px = self.find_parent(x);
        let py = self.find_parent(y);
        self.parent[px] = py;
    }
}

#[allow(clippy::upper_case_acronyms)]
struct DFS<'a> {
    graph: &'a Vec<Vec<(usize, usize)>>,
    link_query: &'a Vec<Vec<(usize, usize)>>,
    vis: Vec<bool>,
    path_from_root: Vec<Value>,
    joint_set: JointSet,
    answer: Vec<i32>,
}

impl<'a> DFS<'a> {
    fn new(
        n: usize,
        m: usize,
        graph: &'a Vec<Vec<(usize, usize)>>,
        link_query: &'a Vec<Vec<(usize, usize)>>,
    ) -> Self {
        DFS {
            graph,
            link_query,
            vis: vec![false; n],
            path_from_root: vec![[0; 26]; n],
            joint_set: JointSet::new(n),
            answer: vec![0; m],
        }
    }

    fn dfs(&mut self, x: usize, path: &mut Value) {
        self.vis[x] = true;
        self.path_from_root[x] = *path;
        for &(y, w) in &self.graph[x] {
            if !self.vis[y] {
                path[w] += 1;
                self.dfs(y, path);
                path[w] -= 1;
                self.joint_set.join(y, x);
            }
        }
        // process the lca
        for &(y, id) in &self.link_query[x] {
            if self.vis[y] {
                let lca = self.joint_set.find_parent(y);
                // println!("{:#?}", self.joint_set.parent);
                // println!("lca of {x} and {y} is {lca}");
                self.answer[id] = self.answer_from_to(x, y, lca);
            }
        }
    }

    fn answer_from_to(&self, x: usize, y: usize, lca: usize) -> i32 {
        let mut path = [0; 26];
        for (i, p) in path.iter_mut().enumerate() {
            *p = self.path_from_root[x][i] + self.path_from_root[y][i]
                - 2 * self.path_from_root[lca][i];
        }
        let s: i32 = path.iter().sum();
        let max: i32 = *path.iter().max().unwrap();
        s - max
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn min_operations_queries(
        n: i32,
        edges: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;
        let m = queries.len();

        let mut graph: Vec<Vec<Edge>> = vec![vec![]; n];
        for e in edges.into_iter() {
            let x = e[0] as usize;
            let y = e[1] as usize;
            let w = e[2] as usize - 1;
            graph[x].push((y, w));
            graph[y].push((x, w));
        }

        let mut link_query: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
        for (id, q) in queries.into_iter().enumerate() {
            let x = q[0] as usize;
            let y = q[1] as usize;
            link_query[x].push((y, id));
            link_query[y].push((x, id));
        }

        let mut dfs = DFS::new(n, m, &graph, &link_query);
        let mut path = [0; 26];
        dfs.dfs(0, &mut path);
        dfs.answer
    }
}

#[test]
fn test1() {
    let n = 7;
    let edges = vec![
        vec![0, 1, 1],
        vec![1, 2, 1],
        vec![2, 3, 1],
        vec![3, 4, 2],
        vec![4, 5, 2],
        vec![5, 6, 2],
    ];
    let queries = vec![vec![0, 3], vec![3, 6], vec![2, 6], vec![0, 6]];
    let ans = vec![0, 0, 1, 3];
    assert_eq!(Solution::min_operations_queries(n, edges, queries), ans);
}

#[test]
fn test2() {
    let n = 2;
    let edges = vec![vec![0, 1, 26]];
    let queries = vec![vec![0, 1], vec![0, 0], vec![1, 1]];
    let ans = vec![0, 0, 0];
    assert_eq!(Solution::min_operations_queries(n, edges, queries), ans);
}
