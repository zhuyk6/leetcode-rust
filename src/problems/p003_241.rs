pub struct Solution;

impl Solution {
    pub fn time_taken(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut graph = vec![vec![]; n];
        for edge in edges {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            graph[u].push(v);
            graph[v].push(u);
        }

        #[inline]
        fn f(x: usize) -> i32 {
            if x & 1 > 0 { 1 } else { 2 }
        }

        fn dfs(x: usize, fa: usize, graph: &Vec<Vec<usize>>, mem: &mut [i32]) -> i32 {
            mem[x] = 0;
            for &y in &graph[x] {
                if y != fa {
                    mem[x] = mem[x].max(f(y) + dfs(y, x, graph, mem));
                }
            }

            mem[x]
        }

        fn dfs2(
            x: usize,
            fa: usize,
            acc: i32,
            graph: &Vec<Vec<usize>>,
            mem: &[i32],
            res: &mut [i32],
        ) -> i32 {
            res[x] = mem[x].max(acc);

            let mut sons = graph[x]
                .iter()
                .copied()
                .filter(|&y| y != fa)
                .map(|y| (y, mem[y] + f(y)))
                .collect::<Vec<_>>();

            sons.sort_unstable_by_key(|(_, v)| *v);

            let m = sons.len();
            match m {
                0 => {}
                1 => {
                    dfs2(sons[0].0, x, acc + f(x), graph, mem, res);
                }
                _ => {
                    let y0 = sons[m - 1].0;

                    // update res[y] except y0
                    let tmp = sons[m - 1].1;
                    let tmp = tmp.max(acc);
                    for &(y, _) in sons.iter().take(m - 1) {
                        dfs2(y, x, tmp + f(x), graph, mem, res);
                    }

                    // update res[y0]
                    let tmp = sons[m - 2].1;
                    let tmp = tmp.max(acc);
                    dfs2(y0, x, tmp + f(x), graph, mem, res);
                }
            }

            mem[x]
        }

        let mut mem = vec![0; n];
        let mut res = vec![0; n];
        dfs(0, usize::MAX, &graph, &mut mem);
        // dbg!(&mem);
        dfs2(0, usize::MAX, 0, &graph, &mem, &mut res);
        res[0] = mem[0];

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let edges = nested_vec![[0, 1], [0, 2]];
        assert_eq!(Solution::time_taken(edges), vec![2, 4, 3]);
    }

    #[test]
    fn sample2() {
        let edges = nested_vec![[0, 1]];
        assert_eq!(Solution::time_taken(edges), vec![1, 2]);
    }

    #[test]
    fn sample3() {
        let edges = nested_vec![[2, 4], [0, 1], [2, 3], [0, 2]];
        assert_eq!(Solution::time_taken(edges), vec![4, 6, 3, 5, 5]);
    }

    #[test]
    fn issue() {
        let edges = nested_vec![[2, 0], [4, 0], [7, 2], [3, 1], [1, 0], [5, 2], [6, 4]];
        assert_eq!(Solution::time_taken(edges), vec![4, 6, 6, 7, 5, 8, 7, 8]);
    }
}
