pub struct Solution;

impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let n = n as usize;

        let mut lines: Vec<(i32, i32)> = ranges
            .iter()
            .enumerate()
            .map(|(i, &v)| (i as i32 - v, i as i32 + v))
            .collect();
        lines.sort_unstable();

        let mut dp = vec![i32::MAX; n + 1];
        for (l, r) in lines {
            if l <= 0 {
                let r = n.min(r as usize);
                for dp_i in dp[..=r].iter_mut() {
                    *dp_i = 1;
                }
            } else {
                let l = l as usize;
                let r = n.min(r as usize);
                let v = dp[l].saturating_add(1);
                for dp_i in dp[l..=r].iter_mut() {
                    if *dp_i > v {
                        *dp_i = v;
                    }
                }
            }
        }

        match dp[n] {
            i32::MAX => -1,
            v => v,
        }
    }
}
