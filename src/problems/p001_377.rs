use std::collections::VecDeque;

pub fn frog_position<Out, In>(n: i32, edges: Out, t: i32, target: i32) -> f64
where
    In: AsRef<[i32]>,
    Out: AsRef<[In]>,
{
    let n = n as usize;
    let t = t as usize;
    let target = target as usize;
    let mut to: Vec<Vec<usize>> = vec![Vec::new(); n + 1];

    for e in edges.as_ref().iter() {
        let e = e.as_ref();
        to[e[0] as usize].push(e[1] as usize);
        to[e[1] as usize].push(e[0] as usize);
    }

    let mut vis = vec![false; n + 1];

    let mut que: VecDeque<(usize, f64, usize)> = VecDeque::new();
    que.push_back((1, 1.0, 0));
    let mut ans: f64 = 0.0;

    while let Some((x, p, now)) = que.pop_front() {
        vis[x] = true;
        if now >= t {
            if x == target {
                ans = p;
                break;
            }
            continue;
        }
        let mut available: usize = 0;
        for y in &to[x] {
            if !vis[*y] {
                available += 1;
            }
        }
        if available == 0 {
            if x == target {
                ans = p;
                break;
            }
        } else {
            for y in &to[x] {
                if !vis[*y] {
                    que.push_back((*y, p / available as f64, now + 1));
                }
            }
        }
    }

    ans
}

#[test]
fn example() {
    let n = 7;
    let edges = [[1, 2], [1, 3], [1, 7], [2, 4], [2, 6], [3, 5]];
    let t = 2;
    let target = 4;

    assert!((frog_position(n, edges, t, target) - 0.16666666).abs() < 1e-5);
}
