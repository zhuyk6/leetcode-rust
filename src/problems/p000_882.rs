struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        let n = n as usize;

        let to = {
            let mut to = vec![vec![]; n];
            for e in edges {
                let x = e[0] as usize;
                let y = e[1] as usize;
                let cnt = e[2];

                to[x].push((y, cnt + 1));
                to[y].push((x, cnt + 1));
            }
            to
        };

        // calculate dist
        let dist = {
            let mut dist = vec![i32::MAX; n];
            let mut vis = vec![false; n];
            dist[0] = 0;

            for _ in 0..(n - 1) {
                let mut x = None;
                for i in 0..n {
                    if !vis[i] && dist[i] < i32::MAX && (x.is_none() || dist[x.unwrap()] > dist[i])
                    {
                        x = Some(i);
                    }
                }

                if let Some(x) = x {
                    vis[x] = true;
                    for &(y, d) in &to[x] {
                        dist[y] = dist[y].min(dist[x] + d);
                    }
                } else {
                    break;
                }
            }
            dist
        };

        eprintln!("dist = {dist:?}");

        let mut indices: Vec<usize> = (0..n).collect();
        indices.sort_unstable_by_key(|i| dist[*i]);

        let mut ans = 0;
        let mut vis = vec![false; n];
        for x in indices {
            if dist[x] > max_moves {
                break;
            }

            vis[x] = true;
            ans += 1;

            for &(y, d) in &to[x] {
                let cnt = d - 1;

                if vis[y] {
                    // dist[y] <= dist[x]
                    if dist[y] + cnt <= max_moves {
                        ans += cnt;
                    } else {
                        let cnt_from_x = max_moves - dist[x];
                        let cnt_from_y = max_moves - dist[y];
                        ans += cnt.min(cnt_from_x + cnt_from_y);
                    }
                } else if dist[y] > max_moves {
                    let cnt_from_x = max_moves - dist[x];
                    ans += cnt_from_x;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let edges = [[0, 1, 10], [0, 2, 1], [1, 2, 2]];
        let max_moves = 6;
        let n = 3;

        let edges: Vec<Vec<i32>> = edges.into_iter().map(Vec::from).collect();

        assert_eq!(Solution::reachable_nodes(edges, max_moves, n), 13);
    }

    #[test]
    fn sample2() {
        let edges = [[0, 1, 4], [1, 2, 6], [0, 2, 8], [1, 3, 1]];
        let max_moves = 10;
        let n = 4;

        let edges: Vec<Vec<i32>> = edges.into_iter().map(Vec::from).collect();

        assert_eq!(Solution::reachable_nodes(edges, max_moves, n), 23);
    }

    #[test]
    fn sample3() {
        let edges = [[1, 2, 4], [1, 4, 5], [1, 3, 1], [2, 3, 4]];
        let max_moves = 17;
        let n = 5;

        let edges: Vec<Vec<i32>> = edges.into_iter().map(Vec::from).collect();

        assert_eq!(Solution::reachable_nodes(edges, max_moves, n), 1);
    }
}
