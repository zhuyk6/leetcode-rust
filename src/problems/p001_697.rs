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
    pub fn distance_limited_paths_exist(
        n: i32,
        edge_list: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let n = n as usize;

        let edge_sorted_indices = {
            let mut v = (0..edge_list.len()).collect::<Vec<_>>();
            v.sort_unstable_by_key(|idx| edge_list[*idx][2]);
            v
        };
        let queries_sorted_indices = {
            let mut v = (0..queries.len()).collect::<Vec<_>>();
            v.sort_unstable_by_key(|idx| queries[*idx][2]);
            v
        };

        let mut ans = vec![false; queries.len()];
        let mut set = JoinSet::with_size(n);
        let mut i = 0;
        for j in queries_sorted_indices {
            while i < edge_list.len() && edge_list[edge_sorted_indices[i]][2] < queries[j][2] {
                set.merge(
                    edge_list[edge_sorted_indices[i]][0] as usize,
                    edge_list[edge_sorted_indices[i]][1] as usize,
                );
                i += 1;
            }
            let f1 = set.get_fa(queries[j][0] as usize);
            let f2 = set.get_fa(queries[j][1] as usize);
            ans[j] = f1 == f2;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let n = 3;
        let edge_list = nested_vec![[0, 1, 2], [1, 2, 4], [2, 0, 8], [1, 0, 16]];
        let queries = nested_vec![[0, 1, 2], [0, 2, 5]];
        assert_eq!(
            Solution::distance_limited_paths_exist(n, edge_list, queries),
            vec![false, true]
        );
    }

    #[test]
    fn sample2() {
        let n = 5;
        let edge_list = nested_vec![[0, 1, 10], [1, 2, 5], [2, 3, 9], [3, 4, 13]];
        let queries = nested_vec![[0, 4, 14], [1, 4, 13]];
        assert_eq!(
            Solution::distance_limited_paths_exist(n, edge_list, queries),
            vec![true, false]
        );
    }
}
