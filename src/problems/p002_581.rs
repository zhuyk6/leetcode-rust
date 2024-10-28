pub struct Solution;

use std::collections::HashSet;

#[allow(clippy::upper_case_acronyms)]
struct DFS<'a> {
    graph: &'a Vec<Vec<usize>>,
    set_query: &'a HashSet<(usize, usize)>,
    bound: usize,
    ans: usize,
}

impl<'a> DFS<'a> {
    fn new(
        graph: &'a Vec<Vec<usize>>,
        set_query: &'a HashSet<(usize, usize)>,
        bound: usize,
    ) -> Self {
        DFS {
            graph,
            set_query,
            bound,
            ans: 0,
        }
    }

    fn dfs1(&self, x: usize, fa: usize) -> usize {
        let mut correct = 0;
        for &y in &self.graph[x] {
            if y != fa {
                correct += self.dfs1(y, x);
            }
        }
        for &y in &self.graph[x] {
            if y != fa && self.set_query.get(&(x, y)).is_some() {
                correct += 1;
            }
        }
        correct
    }

    fn dfs2(&mut self, x: usize, fa: usize, correct: usize) {
        if correct >= self.bound {
            self.ans += 1
        }
        for &y in &self.graph[x] {
            if y != fa {
                let mut tmp = correct;
                if self.set_query.get(&(x, y)).is_some() {
                    tmp -= 1;
                }
                if self.set_query.get(&(y, x)).is_some() {
                    tmp += 1;
                }
                self.dfs2(y, x, tmp);
            }
        }
    }
}

impl Solution {
    pub fn root_count(edges: Vec<Vec<i32>>, guesses: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let n = edges.len() + 1;

        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            graph[x].push(y);
            graph[y].push(x);
        }

        let set_query: HashSet<(usize, usize)> = guesses
            .into_iter()
            .map(|e| (e[0] as usize, e[1] as usize))
            .collect();

        let mut dfs = DFS::new(&graph, &set_query, k);
        let correct0 = dfs.dfs1(0, usize::MAX);
        dfs.dfs2(0, usize::MAX, correct0);
        dfs.ans as i32
    }
}

#[test]
fn test1() {
    let edges = vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![4, 2]];
    let guesses = vec![vec![1, 3], vec![0, 1], vec![1, 0], vec![2, 4]];
    let k = 3;

    assert_eq!(Solution::root_count(edges, guesses, k), 3);
}
