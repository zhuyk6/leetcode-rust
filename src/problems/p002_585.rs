pub struct Solution;

impl Solution {
    pub fn ways_to_reach_target(target: i32, types: Vec<Vec<i32>>) -> i32 {
        const MOD: i32 = 1_000_000_007;

        let t = target as usize;
        let mut dp = vec![vec![0; t + 1]; 2];
        let mut pre = 0;
        let mut cur = 1;
        dp[pre][0] = 1;

        for prob in types {
            let (cnt, score) = (prob[0] as usize, prob[1] as usize);
            dp[cur].fill(0);
            for c in 0..=cnt {
                for i in c * score..=t {
                    dp[cur][i] = (dp[cur][i] + dp[pre][i - c * score]) % MOD;
                }
            }
            // dbg!(&prob);
            // dbg!(&dp[cur]);
            std::mem::swap(&mut cur, &mut pre);
        }
        dp[pre][t]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let target = 6;
        let types = nested_vec![[6, 1], [3, 2], [2, 3]];
        assert_eq!(Solution::ways_to_reach_target(target, types), 7);
    }

    #[test]
    fn sample2() {
        let target = 5;
        let types = nested_vec![[50, 1], [50, 2], [50, 5]];
        assert_eq!(Solution::ways_to_reach_target(target, types), 4);
    }

    #[test]
    fn sample3() {
        let target = 18;
        let types = nested_vec![[6, 1], [3, 2], [2, 3]];
        assert_eq!(Solution::ways_to_reach_target(target, types), 1);
    }
}
