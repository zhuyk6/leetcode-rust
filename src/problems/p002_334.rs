pub struct Solution;

impl Solution {
    pub fn valid_subarray_size(nums: Vec<i32>, threshold: i32) -> i32 {
        let n = nums.len();

        // calculate left boundary
        let left = {
            let mut stk: Vec<(usize, i32)> = Vec::with_capacity(n);
            let mut left = vec![None; n];

            for (i, &v) in nums.iter().enumerate() {
                while let Some(&(j, u)) = stk.last() {
                    if u >= v {
                        stk.pop();
                    } else {
                        left[i] = Some(j);
                        break;
                    }
                }
                stk.push((i, v));
            }
            left
        };

        // calculate right boundary
        let right = {
            let mut stk: Vec<(usize, i32)> = Vec::with_capacity(n);
            let mut right = vec![None; n];

            for (i, &v) in nums.iter().enumerate().rev() {
                while let Some(&(j, u)) = stk.last() {
                    if u >= v {
                        stk.pop();
                    } else {
                        right[i] = Some(j);
                        break;
                    }
                }
                stk.push((i, v));
            }
            right
        };

        dbg!(&left);
        dbg!(&right);

        let mut ans = -1;
        for i in 0..n {
            let mut len = right[i].unwrap_or(n) - i;
            len += left[i].map(|l| i - l - 1).unwrap_or(i);

            if len as i32 * nums[i] > threshold {
                ans = len as i32;
                break;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![1, 3, 4, 3, 1];
        let threshold = 6;
        assert_eq!(Solution::valid_subarray_size(nums, threshold), 3);
    }
}
