pub struct Solution;

struct BinaryIndexTree {
    a: Vec<i32>,
    n: usize,
}

impl BinaryIndexTree {
    fn with_size(n: usize) -> Self {
        let a = vec![0; n];
        Self { a, n }
    }

    #[inline]
    fn lowbit(x: usize) -> usize {
        x & ((!x) + 1)
    }

    fn add(&mut self, pos: usize, delta: i32) {
        if pos == 0 {
            self.a[0] += delta;
        } else {
            let mut x = pos;
            while x < self.n {
                unsafe {
                    *self.a.get_unchecked_mut(x) += delta;
                }
                x += BinaryIndexTree::lowbit(x);
            }
        }
    }

    fn sum(&self, pos: usize) -> i32 {
        let mut x = pos;
        let mut acc = self.a[0];
        while x > 0 {
            unsafe {
                acc += self.a.get_unchecked(x);
            }
            x -= BinaryIndexTree::lowbit(x);
        }
        acc
    }
}

impl Solution {
    pub fn shortest_distance_after_queries_old(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;

        let q_active = {
            use std::cmp::Reverse;
            let mut q_indices: Vec<usize> = (0..queries.len()).collect();
            q_indices.sort_unstable_by_key(|&qid| unsafe {
                let q = queries.get_unchecked(qid);
                (q.get_unchecked(0), Reverse(q.get_unchecked(1)))
            });

            let mut q_active = vec![true; queries.len()];

            let mut r_max: Option<i32> = None;
            let mut qid_min: Option<usize> = None;

            for qid in q_indices {
                let q = unsafe { queries.get_unchecked(qid) };
                if r_max.is_none() || r_max.unwrap() <= q[0] {
                    r_max = Some(q[1]);
                    qid_min = Some(qid);
                } else if qid_min.unwrap() < qid {
                    q_active[qid] = false;
                } else {
                    qid_min = Some(qid);
                }
            }

            q_active
        };

        let mut bit = BinaryIndexTree::with_size(n + 1);

        // initialize
        bit.add(1, (n - 1) as i32);
        for i in 1..n {
            bit.add(i + 1, -1);
        }

        // println!("Now, the nums = ");
        // for i in 0..n {
        //     println!("({}, {})", i, bit.sum(i + 1));
        // }

        let mut ans = Vec::with_capacity(queries.len());

        for (qid, q) in queries.into_iter().enumerate() {
            let u = q[0] as usize;
            let v = q[1] as usize;

            if q_active[qid] {
                let old = bit.sum(u + 1);
                let new = bit.sum(v + 1) + 1;
                let delta = new - old;
                // println!("old = {old}, new = {new}, delta = {delta}");

                bit.add(u + 2, -delta);
                bit.add(0, delta);
            }

            ans.push(bit.sum(1));

            // println!("Now, the nums = ");
            // for i in 0..n {
            //     println!("({}, {})", i, bit.sum(i + 1));
            // }
        }

        ans
    }

    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;

        let mut dist = n - 1;
        let mut next: Vec<usize> = (1..=n).collect();

        // eprintln!("next = {next:?}");

        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            let u = q[0] as usize;
            let v = q[1] as usize;
            if next[u] != usize::MAX && next[v] != usize::MAX {
                let mut x = next[u];
                while x < v {
                    let y = next[x];
                    next[x] = usize::MAX;
                    dist -= 1;
                    x = y;
                }
                next[u] = v;
            }
            ans.push(dist as i32);
            // eprintln!("next = {next:?}");
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
        let n = 5;
        let queries = nested_vec![[2, 4], [0, 2], [0, 4]];
        let ans = vec![3, 2, 1];
        assert_eq!(Solution::shortest_distance_after_queries(n, queries), ans);
    }

    #[test]
    fn sample2() {
        let n = 4;
        let queries = nested_vec![[0, 3], [0, 2]];
        let ans = vec![1, 1];
        assert_eq!(Solution::shortest_distance_after_queries(n, queries), ans);
    }

    #[test]
    fn issue() {
        let n = 8;
        let queries = nested_vec![[1, 4], [4, 7], [2, 4]];
        let ans = vec![5, 3, 3];
        assert_eq!(Solution::shortest_distance_after_queries(n, queries), ans);
    }
}
