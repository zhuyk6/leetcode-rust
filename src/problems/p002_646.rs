pub struct Solution;

struct Tree {
    father: Vec<usize>,
    sons: Vec<Vec<usize>>,
    price: Vec<i32>,
    visit: Vec<u32>,
    depth: Vec<u32>,
}

impl Tree {
    fn build(edges: Vec<Vec<i32>>, price: Vec<i32>) -> Self {
        fn dfs(
            x: usize,
            fa: usize,
            dep: u32,
            to: &[Vec<usize>],
            father: &mut [usize],
            sons: &mut [Vec<usize>],
            depth: &mut [u32],
        ) {
            father[x] = fa;
            depth[x] = dep;
            for &y in &to[x] {
                if y != fa {
                    sons[x].push(y);
                    dfs(y, x, dep + 1, to, father, sons, depth);
                }
            }
        }

        let n = price.len();
        let mut to = vec![vec![]; n];
        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            to[x].push(y);
            to[y].push(x);
        }
        let mut father = vec![0; n];
        let mut sons = vec![vec![]; n];
        let mut depth = vec![0; n];
        dfs(0, usize::MAX, 0, &to, &mut father, &mut sons, &mut depth);

        Tree {
            father,
            sons,
            price,
            visit: vec![0; n],
            depth,
        }
    }

    fn mark(&mut self, mut x: usize, mut y: usize) {
        eprintln!("Mark path {x}-{y}");

        if self.depth[x] < self.depth[y] {
            std::mem::swap(&mut x, &mut y);
        }
        while self.depth[x] > self.depth[y] {
            self.visit[x] += 1;
            x = self.father[x];
        }
        while x != y {
            self.visit[x] += 1;
            self.visit[y] += 1;
            x = self.father[x];
            y = self.father[y];
        }
        self.visit[x] += 1;

        eprintln!("visit = {:?}", self.visit);
    }

    fn mini_cost(&self, x: usize, mark_fa: bool, mem: &mut [[i32; 2]]) -> i32 {
        // using memory
        let tmp = mem[x][mark_fa as usize];
        if tmp != i32::MAX {
            return tmp;
        }
        // don't mark x
        let mut ret = self.visit[x] as i32 * self.price[x];
        for &y in &self.sons[x] {
            ret += self.mini_cost(y, false, mem);
        }

        // mark x
        if !mark_fa {
            let mut ret2 = self.visit[x] as i32 * (self.price[x] / 2);
            for &y in &self.sons[x] {
                ret2 += self.mini_cost(y, true, mem);
            }
            ret = ret.min(ret2);
        }
        mem[x][mark_fa as usize] = ret;
        ret
    }
}

impl Solution {
    pub fn minimum_total_price(
        n: i32,
        edges: Vec<Vec<i32>>,
        price: Vec<i32>,
        trips: Vec<Vec<i32>>,
    ) -> i32 {
        let mut tree = Tree::build(edges, price);
        for trip in trips {
            tree.mark(trip[0] as usize, trip[1] as usize);
        }

        let mut mem = vec![[i32::MAX; 2]; n as usize];
        tree.mini_cost(0, false, &mut mem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let n = 4;
        let edges = nested_vec![[0, 1], [1, 2], [1, 3]];
        let price = nested_vec![2, 2, 10, 6];
        let trips = nested_vec![[0, 3], [2, 1], [2, 3]];
        assert_eq!(Solution::minimum_total_price(n, edges, price, trips), 23);
    }

    #[test]
    fn sample2() {
        let n = 2;
        let edges = nested_vec![[0, 1]];
        let price = nested_vec![2, 2];
        let trips = nested_vec![[0, 0]];
        assert_eq!(Solution::minimum_total_price(n, edges, price, trips), 1);
    }
}
