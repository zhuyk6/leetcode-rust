pub struct Solution;

impl Solution {
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;

        use std::collections::HashMap;
        let mut graph = vec![HashMap::new(); n];
        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            let c = e[2];
            graph[x].entry(y).and_modify(|c0| *c0 &= c).or_insert(c);
            graph[y].entry(x).and_modify(|c0| *c0 &= c).or_insert(c);
        }

        let mut group_num = 0;
        let mut group_cost = vec![];
        let mut belong_group = vec![usize::MAX; n];

        let mut vis = vec![false; n];
        for i in 0..n {
            if !vis[i] {
                use std::collections::VecDeque;
                let mut que = VecDeque::new();
                vis[i] = true;
                que.push_back(i);

                let mut cost = i32::MAX;
                while let Some(x) = que.pop_front() {
                    belong_group[x] = group_num;
                    for (&y, &c) in &graph[x] {
                        cost &= c;
                        if !vis[y] {
                            vis[y] = true;
                            que.push_back(y);
                        }
                    }
                }
                group_cost.push(cost);
                group_num += 1;
            }
        }

        query
            .into_iter()
            .map(|q| {
                let x = q[0] as usize;
                let y = q[1] as usize;
                if belong_group[x] == belong_group[y] {
                    group_cost[belong_group[x]]
                } else {
                    -1
                }
            })
            .collect()
    }
}

#[test]
fn test1() {
    let n = 5;
    let edges = [[0, 1, 7], [1, 3, 7], [1, 2, 1]];
    let query = [[0, 3], [3, 4]];
    let edges: Vec<Vec<i32>> = edges.into_iter().map(Vec::from).collect();
    let query: Vec<Vec<i32>> = query.into_iter().map(Vec::from).collect();

    assert_eq!(Solution::minimum_cost(n, edges, query), vec![1, -1]);
}

#[test]
fn test2() {
    let n = 3;
    let edges = [[0, 2, 7], [0, 1, 15], [1, 2, 6], [1, 2, 1]];
    let query = [[1, 2]];
    let edges: Vec<Vec<i32>> = edges.into_iter().map(Vec::from).collect();
    let query: Vec<Vec<i32>> = query.into_iter().map(Vec::from).collect();

    assert_eq!(Solution::minimum_cost(n, edges, query), vec![0]);
}
