pub struct Solution;

impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i64 {
        let n = nums.len();

        let mut ans = 0;

        // pre[v] : number of {nums[i] : nums[i] <= v, i < j}
        let mut pre = vec![0; n + 1];
        for j in 0..n {
            // println!("j = {j}, pre = {pre:?}");

            // suffix: number of {nums[l] : nums[l] > nums[j], l > k}
            let mut suffix = 0;
            for k in (j + 1..n).rev() {
                if nums[j] > nums[k] {
                    ans += pre[nums[k] as usize] * suffix;
                }
                // println!("k = {k}, suffix = {suffix}, ans = {ans}");
                if nums[k] > nums[j] {
                    suffix += 1;
                }
            }
            // update pre
            #[allow(clippy::needless_range_loop)]
            for v in nums[j] as usize..=n {
                pre[v] += 1;
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
        let nums = vec![1, 3, 2, 4, 5];
        assert_eq!(Solution::count_quadruplets(nums), 2);
    }

    #[test]
    fn sample2() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::count_quadruplets(nums), 0);
    }

    #[test]
    fn sample3() {
        let nums = vec![2, 5, 3, 1, 4];
        assert_eq!(Solution::count_quadruplets(nums), 0);
    }
}
