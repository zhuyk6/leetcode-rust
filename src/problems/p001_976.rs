struct Solution;

const MODP: usize = 1_000_000_007;

impl Solution {
    #[allow(dead_code)]
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
        for e in roads {
            let x = e[0] as usize;
            let y = e[1] as usize;
            let c = e[2] as i64;
            graph[x].push((y, c));
            graph[y].push((x, c));
        }

        let mut dis: Vec<(i64, usize)> = vec![(i64::MAX, 0); n];
        let mut vis: Vec<bool> = vec![false; n];
        dis[0] = (0, 1);
        for _ in 0..n {
            let mut j = n;
            for (i, &dis_i) in dis.iter().enumerate() {
                if !vis[i] && (j == n || dis_i.0 < dis[j].0) {
                    j = i;
                }
            }
            if j == n {
                break;
            }
            vis[j] = true;
            for &(i, c) in &graph[j] {
                if vis[i] {
                    continue;
                }
                match dis[i].0.cmp(&(dis[j].0 + c)) {
                    std::cmp::Ordering::Equal => dis[i].1 = (dis[i].1 + dis[j].1) % MODP,
                    std::cmp::Ordering::Greater => dis[i] = (dis[j].0 + c, dis[j].1),
                    std::cmp::Ordering::Less => (),
                }
            }
        }
        println!("dis: {dis:#?}");

        dis[n - 1].1 as i32
    }
}

#[test]
fn test1() {
    let n = 7;
    let roads = [
        [0, 6, 7],
        [0, 1, 2],
        [1, 2, 3],
        [1, 3, 3],
        [6, 3, 3],
        [3, 5, 1],
        [6, 5, 1],
        [2, 5, 1],
        [0, 4, 5],
        [4, 6, 2],
    ];
    let roads: Vec<Vec<i32>> = roads.into_iter().map(Vec::from).collect();

    assert_eq!(Solution::count_paths(n, roads), 4);
}

#[test]
fn test2() {
    let n = 2;
    let roads = [[1, 0, 10]];
    let roads: Vec<Vec<i32>> = roads.into_iter().map(Vec::from).collect();

    assert_eq!(Solution::count_paths(n, roads), 1);
}
