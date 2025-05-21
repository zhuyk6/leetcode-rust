pub struct Solution;

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        const M: usize = 1000;

        let n = nums.len();
        let m = queries.len();

        let mut dp = vec![vec![false; M + 1]; n];
        for (i, &v) in nums.iter().enumerate() {
            dp[i][v as usize] = true;
        }

        let mut q_id = 0;
        for i in 0..n {
            while !dp[i][0] && q_id < m {
                let (l, r, d) = (
                    queries[q_id][0] as usize,
                    queries[q_id][1] as usize,
                    queries[q_id][2] as usize,
                );

                for j in l.max(i)..=r {
                    if dp[j][0] {
                        continue;
                    }
                    for k in 0.. {
                        if k + d > nums[j] as usize {
                            break;
                        }
                        if dp[j][k + d] {
                            dp[j][k] = true;
                        }
                    }
                }
                q_id += 1;
            }
            if !dp[i][0] {
                return -1;
            }
        }

        q_id as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![2, 0, 2];
        let queries = [[0, 2, 1], [0, 2, 1], [1, 1, 3]];
        let queries: Vec<Vec<i32>> = queries.iter().map(|x| x.to_vec()).collect();
        assert_eq!(Solution::min_zero_array(nums, queries), 2);
    }

    #[test]
    fn sample2() {
        let nums = vec![4, 3, 2, 1];
        let queries = [[1, 3, 2], [0, 2, 1]];
        let queries: Vec<Vec<i32>> = queries.iter().map(|x| x.to_vec()).collect();
        assert_eq!(Solution::min_zero_array(nums, queries), -1);
    }

    #[test]
    fn sample3() {
        let nums = vec![1, 2, 3, 2, 1];
        let queries = [[0, 1, 1], [1, 2, 1], [2, 3, 2], [3, 4, 1], [4, 4, 1]];
        let queries: Vec<Vec<i32>> = queries.iter().map(|x| x.to_vec()).collect();
        assert_eq!(Solution::min_zero_array(nums, queries), 4);
    }

    #[test]
    fn sample4() {
        let nums = vec![1, 2, 3, 2, 6];
        let queries = [
            [0, 1, 1],
            [0, 2, 1],
            [1, 4, 2],
            [4, 4, 4],
            [3, 4, 1],
            [4, 4, 5],
        ];
        let queries: Vec<Vec<i32>> = queries.iter().map(|x| x.to_vec()).collect();
        assert_eq!(Solution::min_zero_array(nums, queries), 4);
    }
}
