pub struct Solution;

const MOVES: [(i32, i32); 8] = [
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (-1, 2),
    (1, -2),
    (1, 2),
    (2, -1),
    (2, 1),
];

type Matrix = Vec<Vec<f64>>;

fn mul(a: &Matrix, b: &Matrix) -> Matrix {
    assert!(a[0].len() == b.len());
    let rows = a.len();
    let mids = a[0].len();
    let cols = b[0].len();

    let mut c = vec![vec![0f64; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            #[allow(clippy::needless_range_loop)]
            for k in 0..mids {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}

impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let m = (n * n) as usize + 1;
        let index = |r: usize, c: usize| -> usize { r * n as usize + c };

        let transfer_mat = {
            let mut mat = vec![vec![0f64; m]; m];
            for i in 0..n {
                for j in 0..n {
                    let fr = index(i as usize, j as usize);
                    for &(dx, dy) in &MOVES {
                        let (x, y) = (i + dx, j + dy);
                        if (0..n).contains(&x) && (0..n).contains(&y) {
                            let to = index(x as usize, y as usize);
                            mat[fr][to] = 1.0 / 8.0;
                        } else {
                            mat[fr][m - 1] += 1.0 / 8.0;
                        }
                    }
                }
            }
            mat[m - 1][m - 1] = 1.0;
            mat
        };

        let s = index(row as usize, column as usize);
        let mut prop = vec![vec![0f64; m]; 1];
        prop[0][s] = 1.0;

        for _ in 0..k {
            prop = mul(&prop, &transfer_mat);
        }

        prop[0].iter().take(m - 1).cloned().sum()
    }
}
