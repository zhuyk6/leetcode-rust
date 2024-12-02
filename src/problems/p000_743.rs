pub struct Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let start = k as usize - 1;

        let mut to: Vec<Vec<(usize, i32)>> = vec![vec![]; n];
        for e in times {
            let x = e[0] as usize - 1;
            let y = e[1] as usize - 1;
            let c = e[2];
            to[x].push((y, c));
        }

        let mut dis = vec![i32::MAX; n];
        let mut vis = vec![false; n];

        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut heap: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::with_capacity(n);

        dis[start] = 0;
        heap.push((Reverse(0), start));

        while let Some((Reverse(dx), x)) = heap.pop() {
            if !vis[x] {
                vis[x] = true;
                for &(y, c) in &to[x] {
                    let dy = dx + c;
                    if dis[y] > dy {
                        dis[y] = dy;
                        heap.push((Reverse(dy), y));
                    }
                }
            }
        }

        match dis.into_iter().max().unwrap() {
            i32::MAX => -1,
            ans => ans,
        }
    }
}
