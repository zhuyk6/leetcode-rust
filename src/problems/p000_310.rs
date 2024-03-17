struct Solution;

struct Dfs<'a> {
    graph: &'a Vec<Vec<usize>>,
    height: Vec<usize>,
    ans: (usize, Vec<usize>),
}

#[allow(unused)]
impl<'a> Dfs<'a> {
    fn new(graph: &'a Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        Dfs {
            graph,
            height: vec![0; n],
            ans: (usize::MAX, vec![]),
        }
    }

    fn dfs1(&mut self, x: usize, fa: usize) -> usize {
        let mut h = 0;
        for &y in &self.graph[x] {
            if y != fa {
                h = h.max(self.dfs1(y, x) + 1);
            }
        }
        self.height[x] = h;
        h
    }

    fn dfs2(&mut self, x: usize, fa: usize, max_from_fa: usize) {
        let mut max1 = max_from_fa; // max height
        let mut max2 = 0; // second max height
        for &y in &self.graph[x] {
            if y != fa {
                let hy = self.height[y] + 1;
                match max1.cmp(&hy) {
                    std::cmp::Ordering::Less => {
                        (max1, max2) = (hy, max1);
                    }
                    std::cmp::Ordering::Equal => {
                        max2 = hy;
                    }
                    std::cmp::Ordering::Greater => {
                        max2 = max2.max(hy);
                    }
                }
            }
        }

        println!("x = {x}, max1 = {max1}, max2 = {max2}");

        // update ans
        match self.ans.0.cmp(&max1) {
            std::cmp::Ordering::Less => {}
            std::cmp::Ordering::Equal => self.ans.1.push(x),
            std::cmp::Ordering::Greater => {
                self.ans.0 = max1;
                self.ans.1.clear();
                self.ans.1.push(x);
            }
        }

        for &y in &self.graph[x] {
            if y != fa {
                let hy = self.height[y] + 1;
                if hy < max1 {
                    self.dfs2(y, x, max1 + 1);
                } else {
                    self.dfs2(y, x, max2 + 1);
                }
            }
        }
    }
}

#[allow(unused)]
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            graph[x].push(y);
            graph[y].push(x);
        }

        let mut dfs = Dfs::new(&graph);

        dfs.dfs1(0, usize::MAX);

        println!("height: {:?}", dfs.height);

        dfs.dfs2(0, usize::MAX, 0);

        dfs.ans.1.into_iter().map(|v| v as i32).collect()
    }
}

#[test]
fn test1() {
    let n = 4;
    let edges = [[1, 0], [1, 2], [1, 3]];
    let mut ans = vec![1];
    ans.sort();
    let edges: Vec<Vec<i32>> = edges.into_iter().map(Vec::from).collect();

    assert_eq!(Solution::find_min_height_trees(n, edges), ans);
}

#[test]
fn test2() {
    let n = 6;
    let edges = [[3, 0], [3, 1], [3, 2], [3, 4], [5, 4]];
    let mut ans = vec![3, 4];
    ans.sort();
    let edges: Vec<Vec<i32>> = edges.into_iter().map(Vec::from).collect();
    assert_eq!(Solution::find_min_height_trees(n, edges), ans);
}
