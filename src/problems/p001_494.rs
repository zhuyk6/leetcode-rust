pub struct Solution;

use std::collections::VecDeque;

#[allow(clippy::needless_range_loop)]
fn check_state(state: usize, graph: &[Vec<usize>]) -> bool {
    let n = graph.len();
    let mut indeg = vec![0u32; n];
    for outer in graph.iter() {
        for &y in outer {
            indeg[y] += 1;
        }
    }

    let mut queue = VecDeque::new();
    for i in 0..n {
        if (state & (1 << i)) != 0 && indeg[i] == 0 {
            queue.push_back(i);
        }
    }

    let mut vis = vec![false; n];
    while let Some(x) = queue.pop_front() {
        vis[x] = true;
        for &y in graph[x].iter() {
            indeg[y] -= 1;
            if (state & (1 << y)) != 0 && indeg[y] == 0 {
                queue.push_back(y);
            }
        }
    }

    for i in 0..n {
        if (state & (1 << i)) != 0 && !vis[i] {
            return false;
        }
    }

    true
}

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn min_number_of_semesters<Outer, Inner>(n: i32, relations: Outer, k: i32) -> i32
    where
        Inner: AsRef<[i32]>,
        Outer: AsRef<[Inner]>,
    {
        let n = n as usize + 1;
        let k = k as u32;

        let graph = {
            let mut graph = vec![vec![]; n];
            let mut indeg = vec![0u32; n];
            for relation in relations.as_ref().iter() {
                let a = relation.as_ref()[0] as usize;
                let b = relation.as_ref()[1] as usize;
                graph[a].push(b);
                indeg[b] += 1;
            }
            for i in 1..n {
                if indeg[i] == 0 {
                    graph[0].push(i);
                }
            }
            graph
        };

        let mut prev = vec![0usize; n];
        for i in 0..n {
            for &j in &graph[i] {
                prev[j] |= 1 << i;
            }
        }

        let is_valid = Vec::from_iter((0..(1 << n)).map(|s| check_state(s, &graph)));

        let can_transfer = |sub: usize, s: usize| -> bool {
            let more = s - sub;
            if more.count_ones() > k {
                return false;
            }

            for i in 0..n {
                if (more & (1 << i)) != 0 && (prev[i] & sub) != prev[i] {
                    return false;
                }
            }

            true
        };

        let mut dp = vec![i32::MAX; 1 << n];

        dp[1] = 0;

        for s in 2..(1 << n) {
            if !is_valid[s] {
                continue;
            }
            let mut sub = s;
            loop {
                sub = (sub - 1) & s;

                if sub == 0 {
                    break;
                }

                if is_valid[sub] && can_transfer(sub, s) {
                    dp[s] = dp[s].min(dp[sub] + 1);
                }
            }
        }

        dp[(1 << n) - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let n = 4;
        let relations = [[2, 1], [3, 1], [1, 4]];
        let k = 2;
        assert_eq!(Solution::min_number_of_semesters(n, relations, k), 3);
    }

    #[test]
    fn sample2() {
        let n = 5;
        let relations = [[2, 1], [3, 1], [4, 1], [1, 5]];
        let k = 2;
        assert_eq!(Solution::min_number_of_semesters(n, relations, k), 4);
    }

    #[test]
    fn sample3() {
        let n = 11;
        let relations: [[i32; 2]; 0] = [];
        let k = 2;
        assert_eq!(Solution::min_number_of_semesters(n, relations, k), 6);
    }

    #[test]
    fn sample4() {
        let n = 4;
        let relations: [[i32; 2]; 3] = [[1, 2], [2, 3], [3, 4]];
        let k = 2;
        assert_eq!(Solution::min_number_of_semesters(n, relations, k), 4);
    }
}
