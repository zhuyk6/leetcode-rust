struct Solution;

#[allow(unused)]
impl Solution {
    pub fn find_whether_exists_path(n: i32, graph: Vec<Vec<i32>>, start: i32, target: i32) -> bool {
        let n = n as usize;

        let mut to = vec![vec![]; n];
        for e in graph {
            let x = e[0] as usize;
            let y = e[1] as usize;
            to[x].push(y);
        }

        let mut vis = vec![false; n];
        use std::collections::VecDeque;
        let mut que = VecDeque::new();

        vis[start as usize] = true;
        que.push_back(start as usize);

        while let Some(x) = que.pop_front() {
            for &y in &to[x] {
                if !vis[y] {
                    vis[y] = true;
                    que.push_back(y);
                }
            }
        }

        vis[target as usize]
    }
}
