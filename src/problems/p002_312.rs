pub struct Solution;

impl Solution {
    pub fn selling_wood(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64 {
        let m = m as usize;
        let n = n as usize;
        let prices: Vec<(usize, usize, i64)> = prices
            .into_iter()
            .map(|v| (v[0] as usize, v[1] as usize, v[2] as i64))
            .filter(|(x, y, _)| *x <= m && *y <= n)
            .collect();

        let mut f = vec![vec![0i64; n + 1]; m + 1];
        for p in &prices {
            f[p.0][p.1] = f[p.0][p.1].max(p.2);
        }

        for i in 1..=m {
            for j in 1..=n {
                // println!("f[{}][{}] = {} ", i, j, f[i][j]);

                // using f[i][j] to update f[i+d][j] and f[i][j+d]
                for d in 1..=(m - i) {
                    f[i + d][j] = f[i + d][j].max(f[i][j] + f[d][j]);
                }
                for d in 1..=(n - j) {
                    f[i][j + d] = f[i][j + d].max(f[i][j] + f[i][d]);
                }
            }
        }

        f[m][n]
    }
}

#[test]
fn test1() {
    let m = 3;
    let n = 5;
    let prices = [[1, 4, 2], [2, 2, 7], [2, 1, 3]];
    let prices: Vec<Vec<i32>> = prices.into_iter().map(Vec::from).collect();

    assert_eq!(Solution::selling_wood(m, n, prices), 19);
}

#[test]
fn test2() {
    let m = 4;
    let n = 6;
    let prices = [[3, 2, 10], [1, 4, 2], [4, 1, 3]];
    let prices: Vec<Vec<i32>> = prices.into_iter().map(Vec::from).collect();

    assert_eq!(Solution::selling_wood(m, n, prices), 32);
}
