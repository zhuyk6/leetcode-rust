pub struct Solution;

const MOVES: [&[usize]; 10] = [
    &[4, 6],
    &[6, 8],
    &[7, 9],
    &[4, 8],
    &[0, 3, 9],
    &[],
    &[0, 1, 7],
    &[2, 6],
    &[1, 3],
    &[2, 4],
];

const MOD: i32 = 1_000_000_007;

type Matrix = Vec<Vec<i32>>;

fn mul(a: &Matrix, b: &Matrix) -> Matrix {
    let m = a.len();
    let mid = a[0].len();
    let n = b[0].len();
    let mut c = vec![vec![0; a.len()]; b[0].len()];
    #[allow(clippy::needless_range_loop)]
    for i in 0..m {
        for j in 0..n {
            for k in 0..mid {
                let tmp = (a[i][k] as i64 * b[k][j] as i64) % MOD as i64;
                c[i][j] = (c[i][j] + tmp as i32) % MOD;
            }
        }
    }
    c
}

fn pow(mut a: Matrix, n: u32) -> Matrix {
    let mut acc = vec![vec![0; a.len()]; a.len()];
    for (i, row) in acc.iter_mut().enumerate() {
        row[i] = 1;
    }

    let mut b = 1;
    while b <= n {
        if n & b > 0 {
            acc = mul(&acc, &a);
        }
        a = mul(&a, &a);
        b <<= 1;
    }
    acc
}

impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let n = n as usize;

        let mut mat = vec![vec![0; 10]; 10];
        for (i, &row) in MOVES.iter().enumerate() {
            for &j in row {
                mat[i][j] = 1;
            }
        }

        let mat = pow(mat, (n - 1) as u32);
        mat.into_iter().flatten().fold(0, |acc, v| (acc + v) % MOD)
    }
}
