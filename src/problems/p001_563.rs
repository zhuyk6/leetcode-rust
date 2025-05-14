pub struct Solution;

impl Solution {
    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        let arr = stone_value;
        let n = arr.len();

        let prefix_sum = {
            let mut prefix_sum = vec![0; n];
            prefix_sum[0] = arr[0];
            for i in 1..n {
                prefix_sum[i] = prefix_sum[i - 1] + arr[i];
            }
            prefix_sum
        };

        let sum = |l: usize, r: usize| {
            assert!(l <= r && r < n);
            if l == 0 {
                prefix_sum[r]
            } else {
                prefix_sum[r] - prefix_sum[l - 1]
            }
        };

        let mut dp = vec![vec![0; n]; n];

        let mut left = vec![vec![0; n]; n];
        let mut right = vec![vec![0; n]; n];
        for i in 0..n {
            left[i][i] = arr[i];
            right[i][i] = arr[i];
        }

        for len in 2..=n {
            for i in 0..=n - len {
                let j = i + len - 1;

                // find the k = max {mid : sum(l, mid-1) <= sum(mid, r)}
                let k = {
                    let mut l = i;
                    let mut r = j;
                    while l < r {
                        let mid = (l + r).div_ceil(2);
                        let l_sum = sum(i, mid - 1);
                        let r_sum = sum(mid, j);
                        if l_sum <= r_sum {
                            l = mid;
                        } else {
                            r = mid - 1;
                        }
                    }
                    l
                };

                // update dp[i][j]
                assert!(k == i || sum(i, k - 1) <= sum(k, j));
                dp[i][j] = if i == k {
                    right[k + 1][j]
                } else if k == j {
                    left[i][k - 1]
                } else {
                    match sum(i, k - 1).cmp(&sum(k, j)) {
                        std::cmp::Ordering::Less => left[i][k - 1].max(right[k + 1][j]),
                        std::cmp::Ordering::Equal => left[i][k - 1].max(right[k][j]),
                        std::cmp::Ordering::Greater => panic!("should not happen"),
                    }
                };

                // update left and right
                let tmp = dp[i][j] + sum(i, j);
                left[i][j] = left[i][j - 1].max(tmp);
                right[i][j] = right[i + 1][j].max(tmp);
            }
        }

        dp[0][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let stone_value = vec![6, 2, 3, 4, 5, 5];
        let result = Solution::stone_game_v(stone_value);
        assert_eq!(result, 18);
    }

    #[test]
    fn sample2() {
        let stone_value = vec![7, 7, 7, 7, 7, 7, 7];
        let result = Solution::stone_game_v(stone_value);
        assert_eq!(result, 28);
    }

    #[test]
    fn sample3() {
        let stone_value = vec![4];
        let result = Solution::stone_game_v(stone_value);
        assert_eq!(result, 0);
    }
}
