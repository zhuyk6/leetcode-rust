use std::vec;

pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
    let d: usize = d as usize;
    let n: usize = job_difficulty.len();
    if n < d {
        return -1;
    }
    let mut f: Vec<Vec<i32>> = vec![vec![i32::MAX; d]; n];

    f[0][0] = job_difficulty[0];
    for i in 1..n {
        f[i][0] = f[i - 1][0].max(job_difficulty[i]);
    }

    for j in 1..d {
        for i in j..n {
            let mut max = job_difficulty[i];
            for k in (0..i).rev() {
                f[i][j] = f[i][j].min(f[k][j - 1].saturating_add(max));
                max = max.max(job_difficulty[k]);
            }
        }
    }
    f[n - 1][d - 1]
}

#[test]
fn example() {
    let jobs = vec![6, 5, 4, 3, 2, 1];
    let d = 2;
    assert_eq!(min_difficulty(jobs, d), 7);

    let jobs = vec![9, 9, 9];
    let d = 4;
    assert_eq!(min_difficulty(jobs, d), -1);

    let jobs = vec![1, 1, 1];
    let d = 3;
    assert_eq!(min_difficulty(jobs, d), 3);
}
