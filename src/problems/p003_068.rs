pub struct Solution;

use std::collections::HashMap;

fn build_tree(n: usize, edges: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
    let mut tree = vec![vec![]; n];
    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        tree[u].push(v);
        tree[v].push(u);
    }
    let mut sons = vec![vec![]; n];

    fn dfs(tree: &Vec<Vec<usize>>, x: usize, fa: usize, sons: &mut Vec<Vec<usize>>) {
        for &y in &tree[x] {
            if y == fa {
                continue;
            }
            sons[x].push(y);
            dfs(tree, y, x, sons);
        }
    }

    dfs(&tree, 0, usize::MAX, &mut sons);
    sons
}

#[derive(Debug, Default)]
struct MemDfs {
    mem: HashMap<(usize, bool), i64>,
}

impl MemDfs {
    fn dfs(&mut self, sons: &[Vec<usize>], nums: &[i32], k: i32, x: usize, set_fa: bool) -> i64 {
        if let Some(&result) = self.mem.get(&(x, set_fa)) {
            return result;
        }

        let mut record = vec![[(0, 0); 2]; sons[x].len()];
        for (i, &y) in sons[x].iter().enumerate() {
            record[i][0] = (self.dfs(sons, nums, k, y, false), 0);
            record[i][1] = (self.dfs(sons, nums, k, y, true), 1);
            record[i].sort();
        }

        let current_node = [nums[x] as i64, nums[x] as i64 ^ k as i64];

        let mut acc = 0i64;
        let mut parity = 0;
        let mut min_gap = i64::MAX;
        for [r0, r1] in &record {
            acc += r1.0;
            parity ^= r1.1;
            min_gap = min_gap.min(r1.0 - r0.0);
        }

        if set_fa {
            parity ^= 1;
        }

        acc += i64::max(
            current_node[parity],
            current_node[parity ^ 1].wrapping_sub(min_gap),
        );

        self.mem.insert((x, set_fa), acc);
        acc
    }
}

impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        let sons = build_tree(nums.len(), edges);
        let mut mem_dfs = MemDfs::default();
        mem_dfs.dfs(&sons, &nums, k, 0, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![1, 2, 1];
        let k = 3;
        let edges = [[0, 1], [0, 2]];
        let edges: Vec<Vec<i32>> = edges.iter().map(|x| x.to_vec()).collect();
        assert_eq!(Solution::maximum_value_sum(nums, k, edges), 6);
    }

    #[test]
    fn sample2() {
        let nums = vec![2, 3];
        let k = 7;
        let edges = [[0, 1]];
        let edges: Vec<Vec<i32>> = edges.iter().map(|x| x.to_vec()).collect();
        assert_eq!(Solution::maximum_value_sum(nums, k, edges), 9);
    }

    #[test]
    fn sample3() {
        let nums = vec![7, 7, 7, 7, 7, 7];
        let k = 3;
        let edges = [[0, 1], [0, 2], [0, 3], [0, 4], [0, 5]];
        let edges: Vec<Vec<i32>> = edges.iter().map(|x| x.to_vec()).collect();
        assert_eq!(Solution::maximum_value_sum(nums, k, edges), 42);
    }
}
