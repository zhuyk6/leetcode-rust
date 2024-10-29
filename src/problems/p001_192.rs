pub struct Solution;

impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut to = vec![vec![]; n];
        for e in connections {
            let x = e[0] as usize;
            let y = e[1] as usize;
            to[x].push(y);
            to[y].push(x);
        }

        let mut index = vec![u32::MAX; n];
        let mut low_index = vec![u32::MAX; n];

        fn dfs(
            x: usize,
            fa: usize,
            to: &[Vec<usize>],
            timestamp: &mut u32,
            index: &mut [u32],
            low_index: &mut [u32],
            critical: &mut Vec<Vec<i32>>,
        ) {
            *timestamp += 1;
            index[x] = *timestamp;
            low_index[x] = *timestamp;

            for &y in &to[x] {
                if y != fa {
                    if index[y] == u32::MAX {
                        dfs(y, x, to, timestamp, index, low_index, critical);
                    }
                    low_index[x] = low_index[x].min(low_index[y]);

                    if index[y] == low_index[y] && index[x] < index[y] {
                        critical.push(vec![x as i32, y as i32]);
                    }
                }
            }
        }

        let mut critical = vec![];
        let mut timestamp = 0;
        dfs(
            0,
            usize::MAX,
            &to,
            &mut timestamp,
            &mut index,
            &mut low_index,
            &mut critical,
        );

        critical
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let n = 4;
        let connections = nested_vec![[0, 1], [1, 2], [2, 0], [1, 3]];
        let mut left = Solution::critical_connections(n, connections);
        left.sort_by_key(|v| (v[0], v[1]));
        let right = nested_vec![[1, 3]];
        assert_eq!(left, right);
    }

    #[test]
    fn sample2() {
        let n = 2;
        let connections = nested_vec![[0, 1]];
        let mut left = Solution::critical_connections(n, connections);
        left.sort_by_key(|v| (v[0], v[1]));
        let right = nested_vec![[0, 1]];
        assert_eq!(left, right);
    }
}
