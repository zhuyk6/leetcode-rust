pub struct Solution;

impl Solution {
    pub fn collect_the_coins(coins: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = coins.len();
        let mut keep = vec![true; n];

        let mut to: Vec<Vec<usize>> = vec![vec![]; n];
        let mut deg = vec![0; n];
        for e in &edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            deg[x] += 1;
            deg[y] += 1;
            to[x].push(y);
            to[y].push(x);
        }

        let mut queue = std::collections::VecDeque::new();
        for (i, d) in deg.iter().enumerate() {
            if *d == 1 {
                queue.push_back(i);
            }
        }

        let mut labeled = vec![];

        while let Some(x) = queue.pop_front() {
            if coins[x] == 0 {
                keep[x] = false;
                for &y in &to[x] {
                    if keep[y] {
                        deg[y] -= 1;
                        if deg[y] < 2 {
                            queue.push_back(y);
                        }
                    }
                }
            } else {
                labeled.push(x);
            }
        }

        // dbg!(&labeled);
        // dbg!(&keep);

        let mut queue = std::collections::VecDeque::new();

        for x in labeled {
            queue.push_back((x, 2u32));
        }

        while let Some((x, d)) = queue.pop_front() {
            if d == 0 {
                continue;
            }
            keep[x] = false;
            for &y in &to[x] {
                if keep[y] {
                    deg[y] -= 1;
                    if deg[y] < 2 {
                        queue.push_back((y, d - 1));
                    }
                }
            }
        }

        // dbg!(&keep);

        let m = keep.into_iter().filter(|v| *v).count();
        if m == 0 { 0 } else { (2 * (m - 1)) as i32 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let coins = vec![1, 0, 0, 0, 0, 1];
        let edges = nested_vec![[0, 1], [1, 2], [2, 3], [3, 4], [4, 5]];
        assert_eq!(Solution::collect_the_coins(coins, edges), 2);
    }

    #[test]
    fn sample2() {
        let coins = vec![0, 0, 0, 1, 1, 0, 0, 1];
        let edges = nested_vec![[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [5, 6], [5, 7]];
        assert_eq!(Solution::collect_the_coins(coins, edges), 2);
    }

    #[test]
    fn sample3() {
        let coins = vec![0, 0, 1, 0, 0, 1];
        let edges = nested_vec![[0, 1], [0, 2], [1, 3], [3, 4], [0, 5]];
        assert_eq!(Solution::collect_the_coins(coins, edges), 0);
    }
}
