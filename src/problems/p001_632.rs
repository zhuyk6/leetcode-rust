pub struct Solution;

struct JoinSet {
    fa: Vec<usize>,
}

impl JoinSet {
    fn new(n: usize) -> Self {
        Self {
            fa: (0..n).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.fa[x] != x {
            self.fa[x] = self.find(self.fa[x]);
        }
        self.fa[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let fx = self.find(x);
        let fy = self.find(y);
        if fx != fy {
            self.fa[fy] = fx;
        }
    }
}

impl Solution {
    pub fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = matrix.len();
        let n = matrix[0].len();

        let flat_index = |i: usize, j: usize| i * n + j;

        let mut join_set = JoinSet::new(m * n);

        let mut edges: Vec<Vec<usize>> = vec![vec![]; m * n];
        let mut in_deg: Vec<u32> = vec![0; m * n];

        // sort elements in each row
        let rows_ordered: Vec<Vec<(i32, usize)>> = matrix
            .iter()
            .map(|row| {
                let mut v: Vec<(i32, usize)> = row
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(j, v)| (v, j))
                    .collect();
                v.sort_unstable();
                v
            })
            .collect();

        for (i, row) in rows_ordered.iter().enumerate() {
            let mut pre_pos = usize::MAX;
            let mut pre_val = i32::MIN;

            for &(val, pos) in row {
                if val == pre_val {
                    join_set.union(flat_index(i, pre_pos), flat_index(i, pos));
                }
                pre_val = val;
                pre_pos = pos;
            }
        }

        // sort elements in each column
        let cols_ordered: Vec<Vec<(i32, usize)>> = (0..n)
            .map(|j| {
                let mut v: Vec<(i32, usize)> = (0..m).map(|i| (matrix[i][j], i)).collect();
                v.sort_unstable();
                v
            })
            .collect();

        for (j, col) in cols_ordered.iter().enumerate() {
            let mut pre_pos = usize::MAX;
            let mut pre_val = i32::MIN;

            for &(val, pos) in col {
                if val == pre_val {
                    join_set.union(flat_index(pre_pos, j), flat_index(pos, j));
                }
                pre_val = val;
                pre_pos = pos;
            }
        }

        // build graph
        for (i, row) in rows_ordered.iter().enumerate() {
            let mut pre_pos = usize::MAX;
            let mut pre_val = i32::MIN;

            for &(val, pos) in row {
                if val > pre_val && pre_pos != usize::MAX {
                    let u = join_set.find(flat_index(i, pre_pos));
                    let v = join_set.find(flat_index(i, pos));
                    if u != v {
                        edges[u].push(v);
                        in_deg[v] += 1;
                    }
                }
                pre_pos = pos;
                pre_val = val;
            }
        }

        for (j, col) in cols_ordered.iter().enumerate() {
            let mut pre_pos = usize::MAX;
            let mut pre_val = i32::MIN;

            for &(val, pos) in col {
                if val > pre_val && pre_pos != usize::MAX {
                    let u = join_set.find(flat_index(pre_pos, j));
                    let v = join_set.find(flat_index(pos, j));
                    if u != v {
                        edges[u].push(v);
                        in_deg[v] += 1;
                    }
                }
                pre_pos = pos;
                pre_val = val;
            }
        }

        let mut rank = vec![1; m * n];

        // use topsort to calculate `rank`
        use std::collections::VecDeque;
        let mut que: VecDeque<usize> = VecDeque::new();
        for (i, &deg) in in_deg.iter().enumerate() {
            if i == join_set.find(i) && deg == 0 {
                que.push_back(i);
            }
        }

        while let Some(u) = que.pop_front() {
            for &v in &edges[u] {
                in_deg[v] -= 1;
                if in_deg[v] == 0 {
                    rank[v] = rank[u] + 1;
                    que.push_back(v);
                }
            }
        }

        let mut ans: Vec<Vec<i32>> = vec![vec![0; n]; m];
        #[allow(clippy::needless_range_loop)]
        for i in 0..m {
            for j in 0..n {
                let g_id = join_set.find(flat_index(i, j));
                ans[i][j] = rank[g_id];
            }
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
        let matrix = nested_vec![[1, 2], [3, 4]];
        assert_eq!(
            Solution::matrix_rank_transform(matrix),
            nested_vec![[1, 2], [2, 3]]
        );
    }

    #[test]
    fn sample2() {
        let matrix = nested_vec![[7, 7], [7, 7]];
        assert_eq!(
            Solution::matrix_rank_transform(matrix),
            nested_vec![[1, 1], [1, 1]]
        );
    }

    #[test]
    fn sample3() {
        let matrix = nested_vec![[20, -21, 14], [-19, 4, 19], [22, -47, 24], [-19, 4, 19]];
        assert_eq!(
            Solution::matrix_rank_transform(matrix),
            nested_vec![[4, 2, 3], [1, 3, 4], [5, 1, 6], [1, 3, 4]]
        );
    }

    #[test]
    fn issue() {
        let matrix = nested_vec![
            [-37, -50, -3, 44],
            [-37, 46, 13, -32],
            [47, -42, -3, -40],
            [-17, -22, -39, 24]
        ];
        assert_eq!(
            Solution::matrix_rank_transform(matrix),
            nested_vec![[2, 1, 4, 6], [2, 6, 5, 4], [5, 2, 4, 3], [4, 3, 1, 5]]
        );
    }
}
