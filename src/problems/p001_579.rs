pub struct Solution;

struct JoinSet {
    fa: Vec<usize>,
}

impl JoinSet {
    fn with_size(n: usize) -> Self {
        let fa = (0..n).collect::<Vec<_>>();
        Self { fa }
    }

    fn get_fa(&mut self, x: usize) -> usize {
        if self.fa[x] != x {
            self.fa[x] = self.get_fa(self.fa[x]);
        }
        self.fa[x]
    }

    fn merge(&mut self, x: usize, y: usize) -> bool {
        let fx = self.get_fa(x);
        let fy = self.get_fa(y);
        if fx != fy {
            self.fa[fx] = fy;
            true
        } else {
            false
        }
    }
}

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize + 1;

        let mut edge_types = [vec![], vec![], vec![]];
        for e in edges {
            let u = e[1] as usize;
            let v = e[2] as usize;
            edge_types[e[0] as usize - 1].push([u, v]);
        }

        let mut ans = 0;
        let mut set1 = JoinSet::with_size(n);
        let mut set2 = JoinSet::with_size(n);

        for &[u, v] in &edge_types[2] {
            set1.merge(u, v);
            if !set2.merge(u, v) {
                ans += 1;
            }
        }

        for &[x, y] in &edge_types[0] {
            if !set1.merge(x, y) {
                ans += 1;
            }
        }

        for &[x, y] in &edge_types[1] {
            if !set2.merge(x, y) {
                ans += 1;
            }
        }

        fn check(set: &mut JoinSet) -> bool {
            let f1 = set.get_fa(1);
            for x in 1..set.fa.len() {
                let fx = set.get_fa(x);
                if fx != f1 {
                    return false;
                }
            }
            true
        }

        if !check(&mut set1) || !check(&mut set2) {
            -1
        } else {
            ans
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let n = 4;
        let edges = nested_vec![
            [3, 1, 2],
            [3, 2, 3],
            [1, 1, 3],
            [1, 2, 4],
            [1, 1, 2],
            [2, 3, 4]
        ];
        assert_eq!(Solution::max_num_edges_to_remove(n, edges), 2);
    }

    #[test]
    fn sample2() {
        let n = 4;
        let edges = nested_vec![[3, 1, 2], [3, 2, 3], [1, 1, 4], [2, 1, 4]];
        assert_eq!(Solution::max_num_edges_to_remove(n, edges), 0);
    }

    #[test]
    fn sample3() {
        let n = 4;
        let edges = nested_vec![[3, 2, 3], [1, 1, 2], [2, 3, 4]];
        assert_eq!(Solution::max_num_edges_to_remove(n, edges), -1);
    }
}
