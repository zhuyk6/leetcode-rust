pub struct Solution;

impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;

        let mut graph = vec![vec![]; n];
        let mut degree = vec![0; n];
        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            graph[x].push(y);
            degree[y] += 1;
        }

        use std::collections::BTreeSet;
        use std::collections::VecDeque;
        let mut que = VecDeque::new();
        let mut ancestors = vec![BTreeSet::new(); n];

        for (i, d) in degree.iter().enumerate() {
            if *d == 0 {
                que.push_back(i);
            }
        }

        while let Some(x) = que.pop_front() {
            for &y in &graph[x] {
                degree[y] -= 1;
                ancestors[y].insert(x);
                let tmp = ancestors[x].clone();
                ancestors[y].extend(tmp);
                if degree[y] == 0 {
                    que.push_back(y);
                }
            }
        }

        ancestors
            .into_iter()
            .map(|v| v.into_iter().map(|x| x as i32).collect())
            .collect()
    }
}
