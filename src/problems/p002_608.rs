pub struct Solution;

impl Solution {
    pub fn find_shortest_cycle(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];

        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            graph[x].push(y);
            graph[y].push(x);
        }

        use std::collections::VecDeque;
        let mut queue = VecDeque::<(usize, usize)>::with_capacity(n);
        let mut dep = vec![i32::MAX; n];

        let mut ans = i32::MAX;
        for start in 0..n {
            queue.clear();
            dep.fill(i32::MAX);

            queue.push_back((start, usize::MAX));
            dep[start] = 1;

            while let Some((x, from)) = queue.pop_front() {
                for &y in &graph[x] {
                    if y == from {
                        continue;
                    }
                    if dep[y] != i32::MAX {
                        ans = ans.min(dep[x] + dep[y] - 1);
                        break;
                    } else {
                        dep[y] = dep[x] + 1;
                        queue.push_back((y, x));
                    }
                }
            }
        }
        match ans {
            i32::MAX => -1,
            _ => ans,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let n = 7;
        let edges = nested_vec![[0, 1], [1, 2], [2, 0], [3, 4], [4, 5], [5, 6], [6, 3]];
        assert_eq!(Solution::find_shortest_cycle(n, edges), 3);
    }

    #[test]
    fn sample2() {
        let n = 4;
        let edges = nested_vec![[0, 1], [0, 2]];
        assert_eq!(Solution::find_shortest_cycle(n, edges), -1);
    }
}
