pub struct Solution;

#[allow(unused, clippy::needless_range_loop)]
impl Solution {
    pub fn remove_boxes(a: Vec<i32>) -> i32 {
        let n = a.len();
        let m = *a.iter().max().unwrap() as usize;

        let mut color_last = vec![usize::MAX; m + 1];
        let mut last_pos = vec![usize::MAX; n];
        for (i, &c) in a.iter().enumerate() {
            last_pos[i] = color_last[c as usize];
            color_last[c as usize] = i;
        }

        let mut f = vec![vec![vec![0; n + 2]; n + 2]; n + 2];
        for i in 0..n {
            f[i][i][1] = 1;
        }

        for l in 2..=n {
            for i in 0..(n + 1 - l) {
                let j = i + l - 1;
                // k = 1
                f[i][j][1] = 1 + f[i][j - 1].iter().max().unwrap();

                // k >= 2
                let mut seq = vec![]; // jump list
                let mut q = last_pos[j];
                while q != usize::MAX && q >= i {
                    seq.push(q);
                    q = last_pos[q];
                }
                let seq: Vec<usize> = seq.into_iter().rev().collect();
                let max_k = seq.len() + 1;

                for k in 2..=max_k {
                    for (t, &q) in seq.iter().enumerate().rev() {
                        if t + 1 < k - 1 {
                            break;
                        }
                        let tmp = *f[q + 1][j - 1].iter().max().unwrap();
                        f[i][j][k] = f[i][j][k].max(f[i][q][k - 1] + tmp + 2 * k as i32 - 1);
                    }
                }
            }
        }

        *f[0][n - 1].iter().max().unwrap()
    }
}

#[test]
fn test1() {
    let a = vec![1, 3, 2, 2, 2, 3, 4, 3, 1];
    assert_eq!(Solution::remove_boxes(a), 23);
}

#[test]
fn test2() {
    let a = vec![1, 1, 1];
    assert_eq!(Solution::remove_boxes(a), 9);
}

#[test]
fn test3() {
    let a = vec![1];
    assert_eq!(Solution::remove_boxes(a), 1);
}

#[test]
fn test4() {
    let a = vec![8, 1, 2, 10, 8, 5, 1, 10, 8, 4];
    assert_eq!(Solution::remove_boxes(a), 16);
}

#[test]
fn test5() {
    let a = vec![
        1, 2, 2, 1, 1, 1, 2, 1, 1, 2, 1, 2, 1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 1, 2, 2, 2, 2, 1, 2, 1,
        1, 2, 2, 1, 2, 1, 2, 2, 2, 2, 2, 1, 2, 1, 2, 2, 1, 1, 1, 2, 2, 1, 2, 1, 2, 2, 1, 2, 1, 1,
        1, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1, 1, 1, 2, 2, 1, 1,
        1, 1, 1, 1, 1, 2, 1, 2, 2, 1,
    ];
    assert_eq!(Solution::remove_boxes(a), 2758);
}
