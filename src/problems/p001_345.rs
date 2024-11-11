pub struct Solution;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();

        let (belong, groups) = {
            let mut indices: Vec<usize> = (0..n).collect();
            indices.sort_unstable_by_key(|&i| unsafe { arr.get_unchecked(i) });

            let mut belong = vec![0; n];
            let mut cc = 0;
            let mut groups = vec![];

            groups.push(vec![indices[0]]);
            for w in indices.windows(2) {
                if arr[w[1]] > arr[w[0]] {
                    groups.push(vec![]);
                    cc += 1;
                }
                belong[w[1]] = cc;
                groups.last_mut().unwrap().push(w[1]);
            }

            (belong, groups)
        };

        // dbg!(&belong);
        // dbg!(&groups);

        use std::collections::VecDeque;
        let mut queue = VecDeque::with_capacity(n);
        let mut vis = vec![false; n];
        let mut vis_g = vec![false; groups.len()];

        queue.push_back((n - 1, 0));
        vis[n - 1] = true;
        while let Some((x, steps)) = queue.pop_front() {
            if x == 0 {
                return steps;
            }
            if x + 1 < n && !vis[x + 1] {
                vis[x + 1] = true;
                queue.push_back((x + 1, steps + 1));
            }
            if x > 0 && !vis[x - 1] {
                vis[x - 1] = true;
                queue.push_back((x - 1, steps + 1));
            }

            if !vis_g[belong[x]] {
                vis_g[belong[x]] = true;
                for &y in &groups[belong[x]] {
                    if !vis[y] {
                        vis[y] = true;
                        queue.push_back((y, steps + 1));
                    }
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let arr = vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404];
        assert_eq!(Solution::min_jumps(arr), 3);
    }

    #[test]
    fn sample2() {
        let arr = vec![7];
        assert_eq!(Solution::min_jumps(arr), 0);
    }

    #[test]
    fn sample3() {
        let arr = vec![7, 6, 9, 6, 9, 6, 9, 7];
        assert_eq!(Solution::min_jumps(arr), 1);
    }
}
