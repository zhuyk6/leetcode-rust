pub struct Solution;

#[derive(Debug)]
struct Dfs {
    n: usize,
    score: Vec<i32>,
    pre_nodes_mask: Vec<usize>,
    mem: Vec<i32>,
}

impl Dfs {
    fn new(n: usize, score: Vec<i32>, pre_nodes_mask: Vec<usize>) -> Self {
        Dfs {
            n,
            score,
            pre_nodes_mask,
            mem: vec![-1; 1 << n],
        }
    }

    fn dfs(&mut self, visited_mask: usize) -> i32 {
        if visited_mask == (1 << self.n) - 1 {
            // All nodes visited
            return 0;
        }
        if self.mem[visited_mask] != -1 {
            // Return cached result
            return self.mem[visited_mask];
        }
        let mut max_profit = 0;
        for i in 0..self.n {
            if visited_mask & (1 << i) == 0
                && self.pre_nodes_mask[i] & visited_mask == self.pre_nodes_mask[i]
            {
                // Node `i` can be visited
                let new_visited_mask = visited_mask | (1 << i);
                let rank = visited_mask.count_ones() as i32 + 1;
                let profit = rank * self.score[i] + self.dfs(new_visited_mask);
                max_profit = max_profit.max(profit);
            }
        }
        self.mem[visited_mask] = max_profit;
        max_profit
    }
}

impl Solution {
    pub fn max_profit(n: i32, edges: Vec<Vec<i32>>, score: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut pre_nodes_mask: Vec<usize> = vec![0; n];
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            pre_nodes_mask[v] |= 1 << u;
        }

        let mut dfs = Dfs::new(n, score, pre_nodes_mask);
        dfs.dfs(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let n = 2;
        let edges = nested_vec![[0, 1]];
        let score = vec![2, 3];
        assert_eq!(Solution::max_profit(n, edges, score), 8);
    }

    #[test]
    fn sample2() {
        let n = 3;
        let edges = nested_vec![[0, 1], [0, 2]];
        let score = vec![1, 6, 3];
        assert_eq!(Solution::max_profit(n, edges, score), 25);
    }
}
