pub struct Solution;

#[allow(unused, clippy::needless_range_loop)]
impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = piles.len();
        let k = k as usize;

        let mut f = vec![vec![0; k + 1]; n];

        // f[0]
        for (i, &v) in piles[0].iter().enumerate() {
            if i >= k {
                break;
            }
            f[0][i + 1] = f[0][i] + v;
        }

        let sum_n: i32 = piles[0].iter().sum();
        let mut sum_n = sum_n as usize;
        for i in 1..n {
            sum_n += piles[i].iter().sum::<i32>() as usize;

            for k1 in 0..=sum_n {
                if k1 > k {
                    break;
                }
                println!("i = {i}, k1 = {k1}");
                f[i][k1] = f[i - 1][k1];

                let mut sum = 0;
                // piles[i] takes k2 items
                for k2 in 1..=k1.min(piles[i].len()) {
                    sum += piles[i][k2 - 1];
                    f[i][k1] = f[i][k1].max(f[i - 1][k1 - k2] + sum);
                }
            }
        }

        f[n - 1][k]
    }
}

#[test]
fn test1() {
    let piles = vec![vec![1, 100, 3], vec![7, 8, 9]];
    let k = 2;

    assert_eq!(Solution::max_value_of_coins(piles, k), 101);
}

#[test]
fn test2() {
    let piles = vec![
        vec![100],
        vec![100],
        vec![100],
        vec![100],
        vec![100],
        vec![100],
        vec![1, 1, 1, 1, 1, 1, 700],
    ];
    let k = 7;

    assert_eq!(Solution::max_value_of_coins(piles, k), 706);
}
