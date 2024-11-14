pub struct Solution;

impl Solution {
    pub fn count_good_nodes(edges: Vec<Vec<i32>>) -> i32 {
        let n = edges.len() + 1;
        let mut to = vec![vec![]; n];
        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            to[x].push(y);
            to[y].push(x);
        }

        fn dfs(to: &[Vec<usize>], x: usize, fa: usize, ans: &mut u32) -> u32 {
            let mut size = 1;
            let mut same: Option<u32> = None;
            let mut good = true;
            for &y in &to[x] {
                if y != fa {
                    let tmp = dfs(to, y, x, ans);
                    if let Some(v) = same {
                        if v != tmp {
                            good = false;
                        }
                    } else {
                        same = Some(tmp);
                    }
                    size += tmp;
                }
            }
            if good {
                *ans += 1;
            }
            size
        }

        let mut ans = 0;
        dfs(&to, 0, usize::MAX, &mut ans);
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let edges = nested_vec![[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]];
        assert_eq!(Solution::count_good_nodes(edges), 7);
    }

    #[test]
    fn sample2() {
        let edges = nested_vec![
            [0, 1],
            [1, 2],
            [2, 3],
            [3, 4],
            [0, 5],
            [1, 6],
            [2, 7],
            [3, 8]
        ];
        assert_eq!(Solution::count_good_nodes(edges), 6);
    }

    #[test]
    fn sample3() {
        let edges = nested_vec![
            [0, 1],
            [1, 2],
            [1, 3],
            [1, 4],
            [0, 5],
            [5, 6],
            [6, 7],
            [7, 8],
            [0, 9],
            [9, 10],
            [9, 12],
            [10, 11]
        ];
        assert_eq!(Solution::count_good_nodes(edges), 12);
    }
}
