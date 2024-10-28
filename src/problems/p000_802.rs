pub fn eventual_safe_nodes<Out, In>(graph: Out) -> Vec<i32>
where
    In: AsRef<[i32]>,
    Out: AsRef<[In]>,
{
    let n = graph.as_ref().len();
    let mut back_edge = vec![vec![]; n];
    let mut degree = vec![0; n];

    for (i, to) in graph.as_ref().iter().enumerate() {
        for j in to.as_ref() {
            back_edge[*j as usize].push(i);
            degree[i] += 1;
        }
    }

    use std::collections::VecDeque;

    let mut que = VecDeque::new();

    for (i, d) in degree.iter().enumerate() {
        if *d == 0 {
            que.push_back(i);
        }
    }
    let mut safe = vec![];
    while let Some(x) = que.pop_front() {
        safe.push(x as i32);
        for y in back_edge[x].iter() {
            degree[*y] -= 1;
            if degree[*y] == 0 {
                que.push_back(*y);
            }
        }
    }
    safe.sort();
    safe
}

#[test]
fn example() {
    let edge = vec![
        vec![1, 2],
        vec![2, 3],
        vec![5],
        vec![0],
        vec![5],
        vec![7],
        vec![],
        vec![],
    ];
    assert_eq!(eventual_safe_nodes(edge), vec![2, 4, 5, 6, 7]);
}
