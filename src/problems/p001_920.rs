pub struct Solution;

impl Solution {
    pub fn build_array(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let m = n as i32;
        for i in 0..n {
            if nums[i] < m {
                // find the circle
                if nums[i] as usize == i {
                    // circle length is 1
                    nums[i] += m;
                } else {
                    let mut i = i;
                    let mut j = nums[i] as usize;
                    let second = nums[i];

                    while nums[i] < m {
                        let next = if nums[j] < m { nums[j] } else { second };
                        nums[i] = next + m;
                        i = j;
                        j = nums[j] as usize;
                    }
                }
            }
        }
        for v in &mut nums {
            *v -= m;
        }

        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![0, 2, 1, 5, 3, 4];
        assert_eq!(Solution::build_array(nums), vec![0, 1, 2, 4, 5, 3]);
    }

    #[test]
    fn sample2() {
        let nums = vec![5, 0, 1, 2, 3, 4];
        assert_eq!(Solution::build_array(nums), vec![4, 5, 0, 1, 2, 3]);
    }
}
