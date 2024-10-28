pub struct Solution;

struct Solver {
    to: Vec<Vec<(usize, i32)>>,
    speed: i32,
}

impl Solver {
    fn new(edges: Vec<Vec<i32>>, speed: i32) -> Self {
        let n = edges.len() + 1;
        let mut to: Vec<Vec<(usize, i32)>> = vec![vec![]; n];

        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            let w = e[2];
            to[x].push((y, w % speed));
            to[y].push((x, w % speed));
        }

        Solver { to, speed }
    }

    fn dfs(&self, x: usize, fa: usize, acc: i32, cnt: &mut i32) {
        if acc == 0 {
            *cnt += 1;
        }
        for &(y, w) in &self.to[x] {
            if y != fa {
                self.dfs(y, x, (acc + w) % self.speed, cnt);
            }
        }
    }

    fn get_ans(&self) -> Vec<i32> {
        let n = self.to.len();

        (0..n)
            .map(|x| {
                let mut v = vec![];
                for &(y, w) in &self.to[x] {
                    let mut cnt = 0;
                    self.dfs(y, x, w, &mut cnt);
                    v.push(cnt);
                }
                eprintln!("x = {x}, v = {v:?}");
                let s: i32 = v.iter().sum();
                let mut ans = 0;
                for c in v {
                    ans += c * (s - c);
                }
                ans / 2
            })
            .collect()
    }
}

impl Solution {
    pub fn count_pairs_of_connectable_servers(edges: Vec<Vec<i32>>, signal_speed: i32) -> Vec<i32> {
        let sol = Solver::new(edges, signal_speed);
        sol.get_ans()
    }
}

#[cfg(test)]
mod tests {
    use ndarray::array;

    use crate::problems::p003_067::Solution;

    #[test]
    fn sample1() {
        let edges = array![[0, 1, 1], [1, 2, 5], [2, 3, 13], [3, 4, 9], [4, 5, 2]];
        let edges: Vec<Vec<i32>> = edges.outer_iter().map(|row| row.to_vec()).collect();
        let signal_speed = 1;
        let ans = vec![0, 4, 6, 6, 4, 0];
        assert_eq!(
            Solution::count_pairs_of_connectable_servers(edges, signal_speed),
            ans
        );
    }

    #[test]
    fn sample2() {
        let edges = array![
            [0, 6, 3],
            [6, 5, 3],
            [0, 3, 1],
            [3, 2, 7],
            [3, 1, 6],
            [3, 4, 2]
        ];
        let signal_speed = 3;
        let ans = vec![2, 0, 0, 0, 0, 0, 2];

        let edges: Vec<Vec<i32>> = edges.outer_iter().map(|row| row.to_vec()).collect();
        assert_eq!(
            Solution::count_pairs_of_connectable_servers(edges, signal_speed),
            ans
        );
    }
}
