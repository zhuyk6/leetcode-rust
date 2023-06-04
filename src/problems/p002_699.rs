use std::mem::swap;

pub fn modified_graph_edges(
    n: i32,
    mut edges: Vec<Vec<i32>>,
    source: i32,
    destination: i32,
    target: i32,
) -> Vec<Vec<i32>> {
    let source = source as usize;
    let destination = destination as usize;
    let n = n as usize;

    use std::collections::HashSet;

    let mut to = vec![vec![]; n];
    let mut modify_set = HashSet::new();
    for (i, e) in edges.iter().enumerate() {
        to[e[0] as usize].push(i);
        to[e[1] as usize].push(i);
        if e[2] == -1 {
            modify_set.insert(i);
        }
    }

    fn path(
        n: usize,
        source: usize,
        edges: &Vec<Vec<i32>>,
        to: &Vec<Vec<usize>>,
        f: &mut Vec<i32>,
        vis: &mut Vec<bool>,
    ) {
        f.fill(i32::MAX);
        vis.fill(false);

        f[source] = 0;
        for _ in 1..n {
            let mut node = None;
            for j in 0..n {
                if !vis[j] {
                    node = node.map_or(Some(j), |k| if f[j] < f[k] { Some(j) } else { Some(k) });
                }
            }
            let x = node.unwrap();
            vis[x] = true;

            println!("x: {}", x);

            for &eid in &to[x] {
                let mut u = edges[eid][0] as usize;
                let mut v = edges[eid][1] as usize;
                if v == x {
                    swap(&mut u, &mut v);
                }
                let w = edges[eid][2];
                if w == -1 {
                    continue;
                }

                f[v] = f[v].min(f[u].saturating_add(w));
            }
        }
    }

    // first path, check whether there exists a way shorter
    let mut f = vec![i32::MAX; n];
    let mut vis = vec![false; n];
    path(n, source, &edges, &to, &mut f, &mut vis);

    if f[destination] < target {
        return vec![];
    }

    println!("yes 1");

    // second path, set all modify edges to 1
    for &eid in &modify_set {
        edges[eid][2] = 1;
    }

    path(n, source, &edges, &to, &mut f, &mut vis);

    println!("f: {:?}", f);

    if f[destination] > target {
        return vec![];
    }

    println!("yes 2");

    // third path, increase modify edges up to target
    {
        let delta = target - f[destination];
        println!("delta: {}", delta);

        let mut g = vec![i32::MAX; n];
        vis.fill(false);

        g[source] = 0;
        for _ in 1..n {
            let mut node = None;
            for j in 0..n {
                if !vis[j] {
                    node = node.map_or(Some(j), |k| if g[j] < g[k] { Some(j) } else { Some(k) });
                }
            }
            let x = node.unwrap();
            vis[x] = true;

            for &eid in &to[x] {
                let mut u = edges[eid][0] as usize;
                let mut v = edges[eid][1] as usize;
                if v == x {
                    swap(&mut u, &mut v);
                }
                if vis[v] {
                    continue;
                }

                if modify_set.contains(&eid) {
                    println!("u: {}, v: {}, f[v]: {}, g[u]: {}", u, v, f[v], g[u]);
                    edges[eid][2] = edges[eid][2].max(delta + f[v] - g[u]);
                }

                g[v] = g[v].min(g[u].saturating_add(edges[eid][2]));
            }
        }
        println!("f: {:?}", f);
        println!("g: {:?}", g);
    }

    edges
}

#[test]
fn example() {
    let n = 4;
    let edges = vec![
        vec![3, 0, -1],
        vec![1, 2, -1],
        vec![2, 3, -1],
        vec![1, 3, 9],
        vec![2, 0, 5],
    ];
    let source = 0;
    let destination = 1;
    let target = 7;

    assert_eq!(
        modified_graph_edges(n, edges, source, destination, target),
        vec![
            vec![3, 0, 5],
            vec![1, 2, 2],
            vec![2, 3, 1],
            vec![1, 3, 9],
            vec![2, 0, 5]
        ]
    );
}
