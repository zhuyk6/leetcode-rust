struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_num_of_marked_indices(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort();
        // println!("{nums:?}");

        let mut ans = 0;
        let m = n / 2;
        let mut j = m;
        for i in 0..m {
            while j < n && 2 * nums[i] > nums[j] {
                j += 1;
            }
            if j < n {
                ans += 2;
                j += 1;
            } else {
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
        let nums = vec![3, 5, 2, 4];
        assert_eq!(Solution::max_num_of_marked_indices(nums), 2);
    }

    #[test]
    fn sample2() {
        let nums = vec![9, 2, 5, 4];
        assert_eq!(Solution::max_num_of_marked_indices(nums), 4);
    }

    #[test]
    fn sample3() {
        let nums = vec![7, 6, 8];
        assert_eq!(Solution::max_num_of_marked_indices(nums), 0);
    }

    #[test]
    fn sample4() {
        let nums = vec![
            42, 83, 48, 10, 24, 55, 9, 100, 10, 17, 17, 99, 51, 32, 16, 98, 99, 31, 28, 68, 71, 14,
            64, 29, 15, 40,
        ];
        assert_eq!(Solution::max_num_of_marked_indices(nums), 26);
    }
}
