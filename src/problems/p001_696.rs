pub struct Solution;

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        // let n = nums.len();
        let k = k as usize;
        use std::collections::VecDeque;

        let mut queue: VecDeque<(usize, i32)> = VecDeque::with_capacity(k);
        // let mut f = vec![i32::MIN; n];
        let mut f = 0i32;

        for (i, v) in nums.iter().enumerate() {
            while let Some(&(x, _)) = queue.front() {
                if x < i.saturating_sub(k) {
                    queue.pop_front();
                } else {
                    break;
                }
            }

            f = v + queue.front().map(|(_, g)| *g).unwrap_or_default();
            while let Some(&(_, g)) = queue.back() {
                if g <= f {
                    queue.pop_back();
                } else {
                    break;
                }
            }
            queue.push_back((i, f));
        }

        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![1, -1, -2, 4, -7, 3];
        let k = 2;
        assert_eq!(Solution::max_result(nums, k), 7);
    }

    #[test]
    fn sample2() {
        let nums = vec![10, -5, -2, 4, 0, 3];
        let k = 3;
        assert_eq!(Solution::max_result(nums, k), 17);
    }

    #[test]
    fn sample3() {
        let nums = vec![1, -5, -20, 4, -1, 3, -6, -3];
        let k = 2;
        assert_eq!(Solution::max_result(nums, k), 0);
    }
}
