use std::{
    collections::HashSet,
    ops::{Add, Mul},
};

pub fn score_of_students(s: String, answers: Vec<i32>) -> i32 {
    // let limit = *answers.iter().max().unwrap();
    let limit = 1000;
    let s = s.as_bytes();
    let n = s.len();

    let mut f = vec![vec![HashSet::<i32>::new(); n]; n];
    let mut g = vec![vec![None; n]; n];

    for (i, &c) in s.iter().enumerate() {
        if c.is_ascii_digit() {
            f[i][i].insert((c - b'0') as i32);
            g[i][i] = Some((c - b'0') as i32);
        }
    }

    for l in (3..=n).step_by(2) {
        for i in 0..n {
            if !s[i].is_ascii_digit() {
                continue;
            }

            let j = i + l - 1;
            if j >= n {
                break;
            }

            for k in (i + 1)..j {
                if !s[k].is_ascii_digit() {
                    let (op, valid): (fn(i32, i32) -> i32, bool) = match s[k] {
                        b'+' => (Add::add, true),
                        b'*' => (
                            Mul::mul,
                            !s[i..k].contains(&b'+') && !s[k + 1..=j].contains(&b'+'),
                        ),
                        _ => panic!("Op should be multiply or add!"),
                    };

                    if valid {
                        g[i][j] = Some(op(g[i][k - 1].unwrap(), g[k + 1][j].unwrap()));
                    }
                    let mut tmp = HashSet::new();
                    for v1 in f[i][k - 1].iter() {
                        for v2 in f[k + 1][j].iter() {
                            let v3 = op(*v1, *v2);
                            if v3 <= limit {
                                tmp.insert(v3);
                            }
                        }
                    }
                    for v in &tmp {
                        f[i][j].insert(*v);
                    }
                }
            }
        }
    }
    let correct = g[0][n - 1].unwrap();

    let ans: Vec<_> = answers
        .into_iter()
        .map(|v| {
            if f[0][n - 1].contains(&v) {
                if v == correct {
                    5
                } else {
                    2
                }
            } else {
                0
            }
        })
        .collect();
    println!("ans: {:?}", ans);

    ans.into_iter().sum()
}

#[test]
fn example() {
    let s = "7+3*1*2".to_string();
    let answers = vec![20, 13, 42];
    assert_eq!(score_of_students(s, answers), 7);
}

#[test]
fn timelimit() {
    let s = "9+9*3+9*6+6*3+9*3+6*9+6*6+6*3+3".to_string();
    let answers = vec![884, 656, 24, 246, 279, 745, 732, 176, 569];
    let _ans = score_of_students(s, answers);
}

#[test]
fn wrong() {
    let s = "9+8*0".to_string();
    let answers = vec![0];
    assert_eq!(score_of_students(s, answers), 2);
}
