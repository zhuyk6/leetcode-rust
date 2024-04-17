struct Solution;

use std::collections::HashSet;

struct Dfs<'a> {
    graph: &'a Vec<Vec<usize>>,
    nums: &'a [i32],
    set: HashSet<i32>,
    vis: Vec<bool>,
}

impl<'a> Dfs<'a> {
    fn new(n: usize, graph: &'a Vec<Vec<usize>>, nums: &'a [i32]) -> Self {
        Dfs {
            graph,
            nums,
            set: HashSet::new(),
            vis: vec![false; n],
        }
    }

    fn dfs(&mut self, x: usize) {
        if self.vis[x] {
            return;
        }
        self.vis[x] = true;
        self.set.insert(self.nums[x]);

        for y in &self.graph[x] {
            self.dfs(*y);
        }
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let n = parents.len();
        let mut ans = vec![1; n];

        let mut graph = vec![vec![]; n];
        for (x, fx) in parents.iter().enumerate() {
            if *fx != -1 {
                let fx = *fx as usize;
                graph[fx].push(x);
            }
        }

        let mut dfs = Dfs::new(n, &graph, &nums);

        if let Some(start) = nums.iter().position(|v| *v == 1) {
            let mut cur = start;
            let mut need = 1;

            loop {
                println!("cur = {cur}");
                dfs.dfs(cur);
                println!("set = {:?}", dfs.set);

                while dfs.set.contains(&need) {
                    need += 1;
                }
                ans[cur] = need;

                if parents[cur] != -1 {
                    cur = parents[cur] as usize;
                } else {
                    break;
                }
            }
        }

        ans
    }
}

#[test]
fn test1() {
    let parents = vec![-1, 0, 0, 2];
    let nums = vec![1, 2, 3, 4];
    let ans = vec![5, 1, 1, 1];
    assert_eq!(Solution::smallest_missing_value_subtree(parents, nums), ans);
}

#[test]
fn test2() {
    let parents = vec![-1, 0, 1, 0, 3, 3];
    let nums = vec![5, 4, 6, 2, 1, 3];
    let ans = vec![7, 1, 1, 4, 2, 1];
    assert_eq!(Solution::smallest_missing_value_subtree(parents, nums), ans);
}

#[test]
fn test3() {
    let parents = vec![-1, 2, 3, 0, 2, 4, 1];
    let nums = vec![2, 3, 4, 5, 6, 7, 8];
    let ans = vec![1, 1, 1, 1, 1, 1, 1];
    assert_eq!(Solution::smallest_missing_value_subtree(parents, nums), ans);
}
