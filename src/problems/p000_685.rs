struct Solution;

use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();

        let (to, from, in_degree, mut out_degree) = {
            let mut in_degree = vec![0; n + 1];
            let mut out_degree = vec![0; n + 1];
            let mut to = vec![vec![]; n + 1];
            let mut from = vec![vec![]; n + 1];
            for e in &edges {
                let x = e[0] as usize;
                let y = e[1] as usize;
                to[x].push(y);
                from[y].push(x);
                in_degree[y] += 1;
                out_degree[x] += 1;
            }
            (to, from, in_degree, out_degree)
        };

        // eprintln!("in degree = {in_degree:?}");

        if in_degree[1..].iter().all(|deg| *deg == 1) {
            // delete any edge in the loop
            let mut que = VecDeque::new();

            for (x, deg) in out_degree.iter().enumerate().skip(1) {
                if *deg == 0 {
                    que.push_back(x);
                }
            }

            let mut other_edges = HashSet::new();

            while let Some(x) = que.pop_front() {
                for &y in &from[x] {
                    out_degree[y] -= 1;
                    other_edges.insert((y, x));

                    if out_degree[y] == 0 {
                        que.push_back(y);
                    }
                }
            }

            // eprintln!("other edges = {other_edges:?}");

            for e in edges.iter().rev() {
                if !other_edges.contains(&(e[0] as usize, e[1] as usize)) {
                    return e.clone();
                }
            }

            panic!("Something wrong!")
        } else {
            let mut root = usize::MAX;
            let mut index = usize::MAX;
            for (i, deg) in in_degree.iter().enumerate().skip(1) {
                if *deg == 0 {
                    root = i;
                } else if *deg == 2 {
                    index = i;
                }
            }

            assert!(root < usize::MAX);
            assert!(index < usize::MAX);

            eprintln!("root = {root}, index = {index}");

            let delete_edges: Vec<_> = edges
                .iter()
                .filter(|e| e[1] as usize == index)
                .map(|e| (e[0] as usize, e[1] as usize))
                .collect();

            eprintln!("delete edges = {delete_edges:?}");

            let check = |e: (usize, usize)| -> bool {
                let mut que = VecDeque::new();
                que.push_back(root);

                let mut cnt = 0;

                while let Some(x) = que.pop_front() {
                    cnt += 1;
                    for &y in &to[x] {
                        if (x, y) != e {
                            que.push_back(y);
                        }
                    }
                }

                eprintln!("try to delete {e:?}, cnt = {cnt}");

                cnt == n
            };

            let b0 = check(delete_edges[0]);
            let b1 = check(delete_edges[1]);

            match (b0, b1) {
                (_, true) => vec![delete_edges[1].0 as i32, delete_edges[1].1 as i32],
                (true, false) => vec![delete_edges[0].0 as i32, delete_edges[0].1 as i32],
                (false, false) => panic!("something wrong"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let edges = [[1, 2], [1, 3], [2, 3]];
        let edges = edges.into_iter().map(Vec::from).collect::<Vec<_>>();
        assert_eq!(
            Solution::find_redundant_directed_connection(edges),
            vec![2, 3]
        );
    }

    #[test]
    fn sample2() {
        let edges = [[1, 2], [2, 3], [3, 4], [4, 1], [1, 5]];
        let edges = edges.into_iter().map(Vec::from).collect::<Vec<_>>();
        assert_eq!(
            Solution::find_redundant_directed_connection(edges),
            vec![4, 1]
        );
    }

    #[test]
    fn wrong1() {
        let edges = [[2, 1], [3, 1], [4, 2], [1, 4]];
        let edges = edges.into_iter().map(Vec::from).collect::<Vec<_>>();
        assert_eq!(
            Solution::find_redundant_directed_connection(edges),
            vec![2, 1]
        );
    }
}
