struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_points(mut nums: Vec<Vec<i32>>) -> i32 {
        nums.sort_by(|a, b| match a[0].cmp(&b[0]) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => a[1].cmp(&b[1]),
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        });

        let mut acc = 0;
        let mut pre = i32::MIN;

        for seg in nums {
            if seg[0] > pre {
                acc += seg[1] - seg[0] + 1;
                pre = seg[1];
            } else if seg[1] > pre {
                acc += seg[1] - pre;
                pre = seg[1];
            }
        }

        acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn sample1() {
        let nums = array![[3, 6], [1, 5], [4, 7]];
        let nums: Vec<Vec<_>> = nums.outer_iter().map(|r| r.to_vec()).collect();
        assert_eq!(Solution::number_of_points(nums), 7);
    }

    #[test]
    fn sample2() {
        let nums = array![[1, 3], [5, 8]];
        let nums: Vec<Vec<_>> = nums.outer_iter().map(|r| r.to_vec()).collect();
        assert_eq!(Solution::number_of_points(nums), 7);
    }
}
