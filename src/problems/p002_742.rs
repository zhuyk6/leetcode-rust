pub struct Solution;

impl Solution {
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        let n = cost.len();

        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;

        for (c, t) in cost.into_iter().zip(time) {
            let t = t as usize;
            if t >= n {
                dp[n] = dp[n].min(c);
            } else {
                for i in ((t + 1)..=n).rev() {
                    dp[i] = dp[i].min(dp[i - t - 1].saturating_add(c));
                }

                #[allow(clippy::needless_range_loop)]
                for i in 1..=t {
                    dp[i] = dp[i].min(c);
                }
            }
        }

        dp[n]
    }
}
