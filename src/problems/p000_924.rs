struct Solution;

#[allow(unused, clippy::needless_range_loop)]
impl Solution {
    pub fn min_malware_spread(graph: Vec<Vec<i32>>, initial: Vec<i32>) -> i32 {
        let n = graph.len();
        use std::collections::{HashSet, VecDeque};

        let initial: HashSet<usize> = initial.into_iter().map(|v| v as usize).collect();

        let mut que = VecDeque::new();
        let mut groups: Vec<Vec<usize>> = vec![];
        let mut num_groups = 0;
        let mut vis: Vec<bool> = vec![false; n];

        let mut ans = (-1, 0);

        for i in 0..n {
            if !vis[i] {
                groups.push(vec![]);
                num_groups += 1;

                que.push_back(i);
                vis[i] = true;
                groups[num_groups - 1].push(i);
                while let Some(x) = que.pop_front() {
                    for y in 0..n {
                        if !vis[y] && graph[x][y] == 1 {
                            vis[y] = true;
                            que.push_back(y);
                            groups[num_groups - 1].push(y);
                        }
                    }
                }

                println!("group {num_groups}: {:?}", groups[num_groups - 1]);

                let mut nodes: Vec<usize> = groups[num_groups - 1]
                    .iter()
                    .filter(|v| initial.contains(v))
                    .cloned()
                    .collect();

                nodes.sort_unstable();
                if let Some(x) = nodes.first() {
                    let cnt = if let Some(x2) = nodes.get(1) {
                        0
                    } else {
                        groups[num_groups - 1].len()
                    };
                    println!("cnt = {cnt}, x = {x}");
                    if cnt as i32 > ans.0 || cnt as i32 == ans.0 && *x < ans.1 {
                        ans = (cnt as i32, *x);
                    }
                }
            }
        }

        ans.1 as i32
    }
}

#[test]
fn test1() {
    let graph = [[1, 1, 0], [1, 1, 0], [0, 0, 1]];
    let initial = vec![0, 1];
    let graph: Vec<Vec<i32>> = graph.into_iter().map(Vec::from).collect();
    assert_eq!(Solution::min_malware_spread(graph, initial), 0);
}

#[test]
fn test2() {
    let graph = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
    let initial = vec![0, 2];
    let graph: Vec<Vec<i32>> = graph.into_iter().map(Vec::from).collect();
    assert_eq!(Solution::min_malware_spread(graph, initial), 0);
}

#[test]
fn test3() {
    let graph = [[1, 1, 1], [1, 1, 1], [1, 1, 1]];
    let initial = vec![1, 2];
    let graph: Vec<Vec<i32>> = graph.into_iter().map(Vec::from).collect();
    assert_eq!(Solution::min_malware_spread(graph, initial), 1);
}

#[test]
fn test4() {
    let graph = [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 1], [0, 0, 1, 1]];
    let initial = vec![3, 1];
    let graph: Vec<Vec<i32>> = graph.into_iter().map(Vec::from).collect();
    assert_eq!(Solution::min_malware_spread(graph, initial), 3);
}
