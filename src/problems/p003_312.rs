pub struct Solution;

impl Solution {
    pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
        const M: i32 = 50_000;
        let mut cnt = vec![0; M as usize + 1];
        for v in nums {
            cnt[v as usize] += 1;
        }

        let mut f = vec![0i64; M as usize + 1];
        for g in (1..=M).rev() {
            let mut acc = 0;
            let mut acc_f = 0;
            let mut gg = g;
            while gg <= M {
                acc += cnt[gg as usize];
                if gg > g {
                    acc_f += f[gg as usize];
                }
                gg += g;
            }
            f[g as usize] = acc * (acc - 1) / 2 - acc_f;
        }

        let prefix_sum: Vec<i64> = {
            let mut s = vec![0; M as usize + 1];
            for i in 1..=M as usize {
                s[i] = s[i - 1] + f[i];
            }
            s
        };

        queries
            .into_iter()
            .map(|q| {
                let pos = prefix_sum.partition_point(|&s| s < q + 1);
                pos as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![2, 3, 4];
        let queries = vec![0, 2, 2];
        assert_eq!(Solution::gcd_values(nums, queries), vec![1, 2, 2])
    }

    #[test]
    fn sample2() {
        let nums = vec![4, 4, 2, 1];
        let queries = vec![5, 3, 1, 0];
        assert_eq!(Solution::gcd_values(nums, queries), vec![4, 2, 1, 1])
    }

    #[test]
    fn sample3() {
        let nums = vec![2, 2];
        let queries = vec![0, 0];
        assert_eq!(Solution::gcd_values(nums, queries), vec![2, 2])
    }
}
