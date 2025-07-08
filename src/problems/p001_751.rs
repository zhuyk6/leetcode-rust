pub struct Solution;

impl Solution {
    pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let n = events.len();
        events.sort_unstable_by_key(|e| e[1]);
        let events = events;

        let mut dp = vec![vec![0; k + 1]; n];

        dp[0][0] = 0;
        dp[0][1] = events[0][2];

        for i in 1..n {
            // not select events[i]
            for c in 1..=k {
                dp[i][c] = dp[i - 1][c];
            }

            // select events[i]
            dp[i][1] = dp[i][1].max(events[i][2]);
            for c in 2..=k {
                let j = events.partition_point(|e| e[1] < events[i][0]);

                if j > 0 {
                    dp[i][c] = dp[i][c].max(dp[j - 1][c - 1] + events[i][2]);
                }
            }
        }

        dp[n - 1].iter().copied().max().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let events = nested_vec![[1, 2, 4], [3, 4, 3], [2, 3, 1]];
        let k = 2;
        assert_eq!(Solution::max_value(events, k), 7);
    }

    #[test]
    fn sample2() {
        let events = nested_vec![[1, 2, 4], [3, 4, 3], [2, 3, 10]];
        let k = 2;
        assert_eq!(Solution::max_value(events, k), 10);
    }
}
