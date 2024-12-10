pub struct Solution;

impl Solution {
    pub fn max_value(nums: Vec<i32>, k: i32) -> i32 {
        let max_k = k as usize;
        let n = nums.len();
        const M: usize = 1 << 7;

        let mut f = vec![vec![vec![false; M]; max_k + 1]; n];

        f[0][1][nums[0] as usize] = true;
        for i in 1..n {
            for k in 1..=max_k {
                for mask in 0..M {
                    f[i][k][mask] = f[i - 1][k][mask];
                }
            }
            f[i][1][nums[i] as usize] = true;
            for k in 2..=max_k {
                for mask in 0..M {
                    if f[i - 1][k - 1][mask] {
                        f[i][k][mask | nums[i] as usize] = true;
                    }
                }
            }
        }

        let mut g = vec![vec![vec![false; M]; max_k + 1]; n];
        g[n - 1][1][nums[n - 1] as usize] = true;
        for i in (0..(n - 1)).rev() {
            for k in 1..=max_k {
                for mask in 0..M {
                    g[i][k][mask] = g[i + 1][k][mask];
                }
            }
            g[i][1][nums[i] as usize] = true;
            for k in 2..=max_k {
                for mask in 0..M {
                    if g[i + 1][k - 1][mask] {
                        g[i][k][mask | nums[i] as usize] = true;
                    }
                }
            }
        }

        let mut ans = 0;
        for i in 0..n - 1 {
            for mask1 in 0..M {
                if f[i][max_k][mask1] {
                    for mask2 in 0..M {
                        if g[i + 1][max_k][mask2] {
                            ans = ans.max(mask1 ^ mask2);
                        }
                    }
                }
            }
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issue1() {
        let nums = vec![2, 6, 7, 9, 11, 15, 2, 4, 6, 8];
        let k = 3;
        assert_eq!(Solution::max_value(nums, k), 13);
    }
}
