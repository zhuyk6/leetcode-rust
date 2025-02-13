pub struct Solution;

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        const N: usize = 32;
        fn update(acc: &mut [u32; N], val: i32, minus: bool) {
            for (i, iter_mut) in acc.iter_mut().enumerate() {
                if val & (1 << i) > 0 {
                    if minus {
                        *iter_mut -= 1;
                    } else {
                        *iter_mut += 1;
                    }
                }
            }
        }

        fn check(acc: &[u32; N], k: i32) -> bool {
            let sum = acc
                .iter()
                .rev()
                .fold(0, |s, &v| (s << 1) | if v > 0 { 1 } else { 0 });
            sum >= k
        }

        let n = nums.len();
        let mut acc = [0u32; N];
        let mut ans = usize::MAX;

        let mut r = 0;
        for (l, &v) in nums.iter().enumerate() {
            if r == 0 || r < l {
                r = l;
                update(&mut acc, v, false);
            }
            while r + 1 < n && !check(&acc, k) {
                r += 1;
                update(&mut acc, nums[r], false);
            }
            if check(&acc, k) {
                ans = ans.min(r - l + 1);
            } else {
                break;
            }
            update(&mut acc, v, true);
        }
        match ans {
            usize::MAX => -1,
            _ => ans as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![1, 2, 3];
        let k = 2;
        assert_eq!(Solution::minimum_subarray_length(nums, k), 1);
    }

    #[test]
    fn sample2() {
        let nums = vec![2, 1, 8];
        let k = 10;
        assert_eq!(Solution::minimum_subarray_length(nums, k), 3);
    }

    #[test]
    fn sample3() {
        let nums = vec![1, 2];
        let k = 0;
        assert_eq!(Solution::minimum_subarray_length(nums, k), 1);
    }

    #[test]
    fn issue() {
        let nums = vec![41];
        let k = 3;
        assert_eq!(Solution::minimum_subarray_length(nums, k), 1);
    }
}
