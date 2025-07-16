pub struct Solution;

impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let n = start_time.len();

        let free_times: Vec<i32> = {
            let mut free_times = Vec::with_capacity(n + 1);
            free_times.push(start_time[0]);
            for (s, t) in start_time[1..].iter().zip(&end_time) {
                free_times.push(s - t);
            }
            free_times.push(event_time - end_time[n - 1]);

            free_times
        };

        let prefix_sum: Vec<i32> = {
            let mut prefix_sum = Vec::with_capacity(free_times.len());
            let mut acc = 0;
            for &time in &free_times {
                acc += time;
                prefix_sum.push(acc);
            }
            prefix_sum
        };

        let mut max_free_time = 0;

        let k = k as usize;
        for i in 0..=n {
            let j = i + k;
            if j > n {
                break;
            }
            let current_free_time = if i > 0 {
                prefix_sum[j] - prefix_sum[i - 1]
            } else {
                prefix_sum[j]
            };
            max_free_time = max_free_time.max(current_free_time);
        }

        max_free_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let event_time = 5;
        let k = 1;
        let start_time = vec![1, 3];
        let end_time = vec![2, 5];
        assert_eq!(
            Solution::max_free_time(event_time, k, start_time, end_time),
            2
        );
    }

    #[test]
    fn sample2() {
        let event_time = 10;
        let k = 1;
        let start_time = vec![0, 2, 9];
        let end_time = vec![1, 4, 10];
        assert_eq!(
            Solution::max_free_time(event_time, k, start_time, end_time),
            6
        );
    }

    #[test]
    fn sample3() {
        let event_time = 5;
        let k = 2;
        let start_time = vec![0, 1, 2, 3, 4];
        let end_time = vec![1, 2, 3, 4, 5];
        assert_eq!(
            Solution::max_free_time(event_time, k, start_time, end_time),
            0
        );
    }
}
