pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn max_removal(nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();

        queries.sort_by_key(|q| (q[0], q[1]));
        let mut heap: BinaryHeap<usize> = BinaryHeap::new();

        let mut diff = vec![0; n];
        diff[0] = nums[0];
        for i in 1..n {
            diff[i] = nums[i] - nums[i - 1];
        }

        let m = queries.len();
        let mut q_id = 0;

        let mut ans = 0;
        let mut acc = 0;
        for i in 0..n {
            while q_id < m && queries[q_id][0] as usize <= i {
                heap.push(queries[q_id][1] as usize);
                q_id += 1;
            }

            acc += diff[i];
            while acc > 0 {
                if let Some(r) = heap.pop() {
                    if r < i {
                        return -1;
                    }
                    ans += 1;
                    acc -= 1;
                    if r + 1 < n {
                        diff[r + 1] += 1;
                    }
                } else {
                    return -1;
                }
            }
        }

        m as i32 - ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![2, 0, 2];
        let queries = [[0, 2], [0, 2], [1, 1]];
        let queries = queries.iter().map(|x| x.to_vec()).collect::<Vec<_>>();
        assert_eq!(Solution::max_removal(nums, queries), 1);
    }

    #[test]
    fn sample2() {
        let nums = vec![1, 1, 1, 1];
        let queries = [[1, 3], [0, 2], [1, 3], [1, 2]];
        let queries = queries.iter().map(|x| x.to_vec()).collect::<Vec<_>>();
        assert_eq!(Solution::max_removal(nums, queries), 2);
    }

    #[test]
    fn sample3() {
        let nums = vec![1, 2, 3, 4];
        let queries = [[0, 3]];
        let queries = queries.iter().map(|x| x.to_vec()).collect::<Vec<_>>();
        assert_eq!(Solution::max_removal(nums, queries), -1);
    }

    #[test]
    fn issue() {
        let nums = vec![0, 3];
        let queries = [[0, 1], [0, 0], [0, 1], [0, 1], [0, 0]];
        let queries = queries.iter().map(|x| x.to_vec()).collect::<Vec<_>>();
        assert_eq!(Solution::max_removal(nums, queries), 2);
    }
}
