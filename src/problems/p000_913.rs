pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
    let to: Vec<Vec<usize>> = graph
        .into_iter()
        .map(|v| v.into_iter().map(|x| x as usize).collect())
        .collect();

    let n = to.len();
    // count degree
    let mut deg: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; n]; n]; 2];
    for i in 0..n {
        for j in 0..n {
            deg[0][i][j] = to[i].len();

            if to[j].contains(&0) {
                deg[1][i][j] = to[j].len() - 1;
            } else {
                deg[1][i][j] = to[j].len();
            }
        }
    }

    use std::collections::VecDeque;

    let mut que: VecDeque<(usize, [usize; 2])> = VecDeque::new();
    // turn: 0 -- mouse
    //       1 -- cat
    // winner: 0 -- mouse
    //         1 -- cat
    //         2 -- unknown
    let mut f = vec![vec![vec![2; n]; n]; 2];

    // cat win
    for i in 1..n {
        que.push_back((0, [i, i]));
        f[0][i][i] = 1;
        que.push_back((1, [i, i]));
        f[1][i][i] = 1;
    }
    // mouse win
    for i in 1..n {
        que.push_back((1, [0, i]));
        f[1][0][i] = 0;
    }

    while let Some((turn, pos)) = que.pop_front() {
        let winner = f[turn][pos[0]][pos[1]];
        println!("turn: {}, pos: {:?}, winner: {}", turn, pos, winner);

        let last_positions = &to[pos[turn ^ 1]];
        for q in last_positions {
            // cat can't goto 0
            if turn == 0 && *q == 0 {
                continue;
            }
            let mut last = pos;
            last[turn ^ 1] = *q;

            // already has answer
            if f[turn ^ 1][last[0]][last[1]] < 2 {
                continue;
            }

            if winner == turn ^ 1 {
                f[turn ^ 1][last[0]][last[1]] = turn ^ 1;
                que.push_back((turn ^ 1, last));
            } else {
                deg[turn ^ 1][last[0]][last[1]] -= 1;
                if deg[turn ^ 1][last[0]][last[1]] == 0 {
                    f[turn ^ 1][last[0]][last[1]] = turn;
                    que.push_back((turn ^ 1, last));
                }
            }
        }
    }

    match f[0][1][2] {
        0 => 1,
        1 => 2,
        2 => 0,
        _ => panic!("invalid valid"),
    }
}

#[test]
fn example1() {
    let graph = vec![
        vec![2, 5],
        vec![3],
        vec![0, 4, 5],
        vec![1, 4, 5],
        vec![2, 3],
        vec![0, 2, 3],
    ];
    assert_eq!(cat_mouse_game(graph), 0);
}
#[test]
fn example2() {
    let graph = vec![vec![1, 3], vec![0], vec![3], vec![0, 2]];
    assert_eq!(cat_mouse_game(graph), 1);
}
#[test]
fn wrong() {
    let graph = vec![vec![2, 3], vec![3, 4], vec![0, 4], vec![0, 1], vec![1, 2]];
    assert_eq!(cat_mouse_game(graph), 1);
}
