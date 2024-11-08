pub struct Solution;

#[derive(Debug)]
struct Graph {
    to: Vec<Vec<usize>>,
}

impl Graph {
    fn with_size(n: usize) -> Self {
        let to = vec![vec![]; n];
        Self { to }
    }

    fn add_edge(&mut self, x: usize, y: usize) {
        self.to[x].push(y);
        self.to[y].push(x);
    }

    fn connected_components(&self) -> (u32, Vec<u32>) {
        let mut cc = 0;
        let n = self.to.len();
        let mut vis = vec![false; n];
        let mut belong = vec![u32::MAX; n];

        fn dfs(graph: &Graph, x: usize, vis: &mut [bool], cc: u32, belong: &mut [u32]) {
            vis[x] = true;
            belong[x] = cc;
            for &y in &graph.to[x] {
                if !vis[y] {
                    dfs(graph, y, vis, cc, belong);
                }
            }
        }

        for x in 0..n {
            if !vis[x] {
                dfs(self, x, &mut vis, cc, &mut belong);
                cc += 1;
            }
        }
        (cc, belong)
    }
}

impl Solution {
    pub fn can_reach_corner(x_corner: i32, y_corner: i32, circles: Vec<Vec<i32>>) -> bool {
        let n = circles.len();
        let mut graph = Graph::with_size(n + 4);

        #[inline]
        fn dist_sqr(p0: (i32, i32), p1: (i32, i32)) -> i64 {
            (p0.0 as i64 - p1.0 as i64).pow(2) + (p0.1 as i64 - p1.1 as i64).pow(2)
        }

        #[inline]
        fn circle_join(c1: (i32, i32), r1: i32, c2: (i32, i32), r2: i32) -> bool {
            let d_sqr = dist_sqr(c1, c2);
            let r_sqr = ((r1 + r2) as i64).pow(2);
            d_sqr <= r_sqr
        }

        #[inline]
        fn circle_segment_joins(c: (i32, i32), r: i32, p0: (i32, i32), p1: (i32, i32)) -> bool {
            let d0 = dist_sqr(c, p0);
            let d1 = dist_sqr(c, p1);
            let d2: i64 = if p0.0 == p1.0 {
                if p0.1.min(p1.1) <= c.1 && c.1 <= p0.1.max(p1.1) {
                    ((c.0 - p0.0) as i64).abs().pow(2)
                } else {
                    i64::MAX
                }
            } else if p0.1 == p1.1 {
                if p0.0.min(p1.0) <= c.0 && c.0 <= p0.0.max(p1.0) {
                    ((c.1 - p0.1) as i64).abs().pow(2)
                } else {
                    i64::MAX
                }
            } else {
                panic!("error!")
            };
            d0.min(d1).min(d2) <= (r as i64).pow(2)
        }

        for i in 0..n {
            for j in i + 1..n {
                if circle_join(
                    (circles[i][0], circles[i][1]),
                    circles[i][2],
                    (circles[j][0], circles[j][1]),
                    circles[j][2],
                ) {
                    let x1 = circles[i][0] as i64;
                    let y1 = circles[i][1] as i64;
                    let r1 = circles[i][2] as i64;

                    let x2 = circles[j][0] as i64;
                    let y2 = circles[j][1] as i64;
                    let r2 = circles[j][2] as i64;

                    if x1 * r2 + x2 * r1 <= (r1 + r2) * x_corner as i64
                        && y1 * r2 + y2 * r1 <= (r1 + r2) * y_corner as i64
                    {
                        graph.add_edge(i, j);
                    }
                }
            }
        }

        for (i, circ) in circles.iter().enumerate() {
            let c = (circ[0], circ[1]);
            let r = circ[2];

            // lower -- n
            if circle_segment_joins(c, r, (0, 0), (x_corner, 0)) {
                graph.add_edge(i, n);
            }

            // upper -- n + 2
            if circle_segment_joins(c, r, (0, y_corner), (x_corner, y_corner)) {
                graph.add_edge(i, n + 2);
            }

            // right -- n + 1
            if circle_segment_joins(c, r, (x_corner, 0), (x_corner, y_corner)) {
                graph.add_edge(i, n + 1);
            }

            // left  -- n + 3
            if circle_segment_joins(c, r, (0, 0), (0, y_corner)) {
                graph.add_edge(i, n + 3);
            }
        }

        // dbg!(&graph);
        let (_cc, belong) = graph.connected_components();

        !(belong[n] == belong[n + 2]
            || belong[n + 1] == belong[n + 3]
            || belong[n] == belong[n + 3]
            || belong[n + 1] == belong[n + 2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let x_corner = 3;
        let y_corner = 4;
        let circles = nested_vec![[2, 1, 1]];
        assert!(Solution::can_reach_corner(x_corner, y_corner, circles));
    }

    #[test]
    fn sample2() {
        let x_corner = 3;
        let y_corner = 3;
        let circles = nested_vec![[1, 1, 2]];
        assert!(!Solution::can_reach_corner(x_corner, y_corner, circles));
    }

    #[test]
    fn sample3() {
        let x_corner = 3;
        let y_corner = 3;
        let circles = nested_vec![[2, 1, 1], [1, 2, 1]];
        assert!(!Solution::can_reach_corner(x_corner, y_corner, circles));
    }

    #[test]
    fn sample4() {
        let x_corner = 4;
        let y_corner = 4;
        let circles = nested_vec![[5, 5, 1]];
        assert!(Solution::can_reach_corner(x_corner, y_corner, circles));
    }

    #[test]
    fn issue() {
        let x_corner = 3;
        let y_corner = 3;
        let circles = nested_vec![[2, 1000, 997], [1000, 2, 997]];
        assert!(Solution::can_reach_corner(x_corner, y_corner, circles));
    }
}
