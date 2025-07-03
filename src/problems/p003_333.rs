pub struct Solution;

const MOD: i32 = 1_000_000_007;

fn add_mod(a: i32, b: i32) -> i32 {
    (a + b) % MOD
}

fn sub_mod(a: i32, b: i32) -> i32 {
    ((a - b) % MOD + MOD) % MOD
}

fn mul_mod(a: i32, b: i32) -> i32 {
    (a as i64 * b as i64 % MOD as i64) as i32
}

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn possible_string_count(word: String, k: i32) -> i32 {
        let k = k as usize;
        let mut cnt = vec![1];
        for w in word.as_bytes().windows(2) {
            if w[0] == w[1] {
                *cnt.last_mut().unwrap() += 1;
            } else {
                cnt.push(1);
            }
        }

        // println!("Character counts: {:?}", cnt);

        let n = cnt.len();
        // let mut dp = vec![vec![0; k + 1]; n];
        let mut pre = vec![0; k + 1];
        let mut cur = vec![0; k + 1];

        for j in 1..=cnt[0].min(k) {
            cur[j] = 1;
        }
        if cnt[0] > k {
            cur[k] += (cnt[0] - k) as i32;
        }

        // println!("dp[0]: {:?}", dp[0]);

        for i in 1..n {
            std::mem::swap(&mut pre, &mut cur);
            cur.fill(0);

            // // cnt[i]
            // for j in 1..=cnt[i] {
            //     for l in 1..=k {
            //         if j + l <= k {
            //             dp[i][j + l] = add_mod(dp[i][j + l], dp[i - 1][l]);
            //         } else {
            //             dp[i][k] = add_mod(dp[i][k], dp[i - 1][l]);
            //         }
            //     }
            // }
            // println!("dp[{}]: {:?}", i, dp[i]);
            // dp[i].fill(0);

            // for j in 1..k {
            //     for c in 1..=cnt[i].min(j) {
            //         dp[i][j] = add_mod(dp[i][j], dp[i - 1][j - c]);
            //     }
            // }

            // cur[1] = pre[0];
            // for j in 2..k {
            //     cur[j] = if j <= cnt[i] {
            //         add_mod(cur[j - 1], pre[j - 1])
            //     } else {
            //         sub_mod(add_mod(cur[j - 1], pre[j - 1]), pre[j - cnt[i] - 1])
            //     }
            // }
            for j in i + 1..k {
                cur[j] = if j <= cnt[i] {
                    add_mod(pre[j - 1], cur[j - 1])
                } else {
                    sub_mod(add_mod(pre[j - 1], cur[j - 1]), pre[j - cnt[i] - 1])
                }
            }

            // for c in 1..=cnt[i] {
            //     for j in k.saturating_sub(c)..=k {
            //         dp[i][k] = add_mod(dp[i][k], dp[i - 1][j]);
            //     }
            // }

            cur[k] = mul_mod(pre[k], cnt[i] as i32);
            for c in 1..=cnt[i] {
                let j = k.saturating_sub(c);
                let tmp = (cnt[i] - c + 1) as i32;
                cur[k] = add_mod(cur[k], mul_mod(pre[j], tmp));
            }

            // println!("dp[{}]: {:?}", i, dp[i]);
        }

        cur[k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let word = "aabbccdd".into();
        let k = 7;
        assert_eq!(Solution::possible_string_count(word, k), 5);
    }

    #[test]
    fn sample2() {
        let word = "aabbccdd".into();
        let k = 8;
        assert_eq!(Solution::possible_string_count(word, k), 1);
    }

    #[test]
    fn sample3() {
        let word = "aaabbb".into();
        let k = 3;
        assert_eq!(Solution::possible_string_count(word, k), 8);
    }

    #[test]
    fn sample4() {
        let word = "aabbccddeeee".into();
        let k = 7;
        assert_eq!(Solution::possible_string_count(word, k), 58);
    }

    #[test]
    fn sample5() {
        let word = "aaabbb".into();
        let k = 2;
        assert_eq!(Solution::possible_string_count(word, k), 9);
    }
}
