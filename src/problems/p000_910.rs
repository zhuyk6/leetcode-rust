pub struct Solution;

impl Solution {
    pub fn smallest_range_ii(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut ans =
            nums.last().expect("nums can't be empty") - nums.first().expect("nums can't be empty");

        let n = nums.len();
        for i in 1..n {
            let l_min = nums[0] + k;
            let l_max = nums[i - 1] + k;
            let r_min = nums[i] - k;
            let r_max = nums[n - 1] - k;

            eprintln!("[{l_min}, {l_max}]");
            eprintln!("[{r_min}, {r_max}]");

            let tmp = l_max.max(r_max) - l_min.min(r_min);
            eprintln!("{tmp}");
            ans = ans.min(tmp);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![1];
        let k = 0;
        assert_eq!(Solution::smallest_range_ii(nums, k), 0);
    }

    #[test]
    fn sample2() {
        let nums = vec![0, 10];
        let k = 2;
        assert_eq!(Solution::smallest_range_ii(nums, k), 6);
    }

    #[test]
    fn sample3() {
        let nums = vec![1, 3, 6];
        let k = 3;
        assert_eq!(Solution::smallest_range_ii(nums, k), 3);
    }

    #[test]
    fn wrong1() {
        let nums = vec![3, 1, 10];
        let k = 4;
        assert_eq!(Solution::smallest_range_ii(nums, k), 2);
    }
}
