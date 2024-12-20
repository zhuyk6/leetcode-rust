pub struct Solution;

impl Solution {
    pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        let mut sorted_indices = (0..nums.len()).collect::<Vec<_>>();
        sorted_indices.sort_unstable_by_key(|&i| nums[i]);

        let mut left_cost = 0i64;
        let mut right_cost = cost.iter().fold(0i64, |acc, &v| acc + v as i64);

        let n = nums.len();
        let mut opt = i32::MAX;
        let mut i = 0;
        while i < n {
            left_cost += cost[sorted_indices[i]] as i64;
            right_cost -= cost[sorted_indices[i]] as i64;
            while i + 1 < n && nums[sorted_indices[i]] == nums[sorted_indices[i + 1]] {
                i += 1;
                left_cost += cost[sorted_indices[i]] as i64;
                right_cost -= cost[sorted_indices[i]] as i64;
            }
            if left_cost >= right_cost {
                opt = nums[sorted_indices[i]];
                break;
            }
            i += 1;
        }
        assert!(opt < i32::MAX);
        nums.into_iter().zip(cost).fold(0i64, |acc, (v, c)| {
            acc + ((v - opt).abs() as i64) * c as i64
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![1, 3, 5, 2];
        let cost = vec![2, 3, 1, 14];
        assert_eq!(Solution::min_cost(nums, cost), 8);
    }

    #[test]
    fn sample2() {
        let nums = vec![2, 2, 2, 2, 2];
        let cost = vec![4, 2, 8, 1, 3];
        assert_eq!(Solution::min_cost(nums, cost), 0);
    }
}
