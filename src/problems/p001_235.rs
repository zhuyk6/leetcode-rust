pub struct Solution;

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();

        let mut arr_time = vec![];
        for t in &start_time {
            arr_time.push(*t);
        }
        for t in &end_time {
            arr_time.push(*t);
        }
        arr_time.sort_unstable();
        arr_time.dedup();

        let m = arr_time.len();

        use std::collections::HashMap;
        let time_to_index: HashMap<i32, usize> =
            HashMap::from_iter(arr_time.into_iter().enumerate().map(|(i, v)| (v, i)));

        let mut tasks: Vec<(usize, usize, i32)> = (0..n)
            .map(|i| {
                (
                    *time_to_index.get(&start_time[i]).unwrap(),
                    *time_to_index.get(&end_time[i]).unwrap(),
                    profit[i],
                )
            })
            .collect();
        tasks.sort_unstable();

        println!("tasks: {tasks:?}");

        let mut f = vec![0; m];
        let mut last = 0;
        for (s, t, p) in tasks {
            while last < s {
                f[last + 1] = f[last + 1].max(f[last]);
                last += 1;
            }
            f[t] = f[t].max(f[s] + p);
        }
        while last < m - 1 {
            f[last + 1] = f[last + 1].max(f[last]);
            last += 1;
        }
        f[m - 1]
    }
}

#[test]
fn test1() {
    let start_time = vec![1, 2, 3, 3];
    let end_time = vec![3, 4, 5, 6];
    let profit = vec![50, 10, 40, 70];
    assert_eq!(Solution::job_scheduling(start_time, end_time, profit), 120);
}

#[test]
fn test2() {
    let start_time = vec![1, 2, 3, 4, 6];
    let end_time = vec![3, 5, 10, 6, 9];
    let profit = vec![20, 20, 100, 70, 60];
    assert_eq!(Solution::job_scheduling(start_time, end_time, profit), 150);
}

#[test]
fn test3() {
    let start_time = vec![1, 1, 1];
    let end_time = vec![2, 3, 4];
    let profit = vec![5, 6, 4];
    assert_eq!(Solution::job_scheduling(start_time, end_time, profit), 6);
}
