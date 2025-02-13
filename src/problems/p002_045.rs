pub struct Solution;

impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for e in edges {
            let u = e[0] as usize - 1;
            let v = e[1] as usize - 1;
            graph[u].push(v);
            graph[v].push(u);
        }

        let go = |d: i32| -> i32 {
            match (d / change) % 2 {
                0 => d,
                _ => d + change - d % change,
            }
        };

        let mut d1 = vec![i32::MAX; n];
        let mut d2 = vec![i32::MAX; n];
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut queue: BinaryHeap<Reverse<(i32, usize)>> = std::collections::BinaryHeap::new();

        d1[0] = 0;
        queue.push(Reverse((0i32, 0usize)));
        while let Some(Reverse((dx, x))) = queue.pop() {
            if dx > d2[x] {
                continue;
            }
            let d = go(dx);
            for &y in &graph[x] {
                let dy = d + time;
                if d1[y] > dy {
                    d2[y] = d1[y];
                    d1[y] = dy;
                    queue.push(Reverse((dy, y)));
                } else if d2[y] > dy && dy > d1[y] {
                    d2[y] = dy;
                    queue.push(Reverse((dy, y)));
                }
            }
        }

        // // print
        // for (x, (d1, d2)) in d1.iter().zip(&d2).enumerate() {
        //     println!("d1[{0}] = {1}, d2[{0}] = {2}", x, d1, d2);
        // }

        d2[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let n = 5;
        let edges = nested_vec![[1, 2], [1, 3], [1, 4], [3, 4], [4, 5]];
        let time = 3;
        let change = 5;
        assert_eq!(Solution::second_minimum(n, edges, time, change), 13);
    }

    #[test]
    fn sample2() {
        let n = 2;
        let edges = nested_vec![[1, 2]];
        let time = 3;
        let change = 2;
        assert_eq!(Solution::second_minimum(n, edges, time, change), 11);
    }

    #[test]
    fn issue() {
        let n = 7;
        let edges = nested_vec![
            [1, 2],
            [1, 3],
            [2, 5],
            [2, 6],
            [6, 5],
            [5, 7],
            [3, 4],
            [4, 7]
        ];
        let time = 4;
        let change = 7;
        assert_eq!(Solution::second_minimum(n, edges, time, change), 22);
    }
}
