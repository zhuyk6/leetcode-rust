pub struct Solution;

const MAXVAL: usize = 50;

fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

struct Dfs<'a> {
    nums: &'a Vec<i32>,
    graph: &'a Vec<Vec<usize>>,
    ans: Vec<usize>,
    stack: Vec<Vec<usize>>,
}

impl<'a> Dfs<'a> {
    fn new(nums: &'a Vec<i32>, graph: &'a Vec<Vec<usize>>) -> Self {
        let n = nums.len();

        Dfs {
            nums,
            graph,
            ans: vec![0; n],
            stack: vec![vec![]; MAXVAL + 1],
        }
    }

    fn dfs(&mut self, x: usize, fa: usize) {
        let co_primes: Vec<usize> = (1..=MAXVAL)
            .filter(|v| gcd(self.nums[x], *v as i32) == 1)
            .collect();

        self.ans[x] = *self.stack[self.nums[x] as usize]
            .last()
            .unwrap_or(&usize::MAX);

        for &v in &co_primes {
            self.stack[v].push(x);
        }

        for &y in &self.graph[x] {
            if y != fa {
                self.dfs(y, x);
            }
        }

        for &v in &co_primes {
            self.stack[v].pop();
        }
    }
}

impl Solution {
    pub fn get_coprimes(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();

        let mut graph = vec![vec![]; n];
        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            graph[x].push(y);
            graph[y].push(x);
        }

        let mut dfs = Dfs::new(&nums, &graph);
        dfs.dfs(0, usize::MAX);

        dfs.ans
            .into_iter()
            .map(|v| match v {
                usize::MAX => -1,
                v => v as i32,
            })
            .collect()
    }
}

#[test]
fn test1() {
    let nums = vec![2, 3, 3, 2];
    let edges = [[0, 1], [1, 2], [1, 3]];
    let edges: Vec<Vec<i32>> = edges.into_iter().map(Vec::from).collect();
    let ans = vec![-1, 0, 0, 1];
    assert_eq!(Solution::get_coprimes(nums, edges), ans);
}

#[test]
fn test2() {
    let nums = vec![5, 6, 10, 2, 3, 6, 15];
    let edges = [[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]];
    let edges: Vec<Vec<i32>> = edges.into_iter().map(Vec::from).collect();
    let ans = vec![-1, 0, -1, 0, 0, 0, -1];
    assert_eq!(Solution::get_coprimes(nums, edges), ans);
}
