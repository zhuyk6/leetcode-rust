pub struct Solution;

impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let left_min = {
            let mut ans = vec![None; n];
            let mut stack: Vec<i32> = vec![];
            for (i, v) in nums.iter().enumerate() {
                while let Some(u) = stack.last() {
                    if u >= v {
                        stack.pop();
                    } else {
                        break;
                    }
                }
                ans[i] = stack.first().cloned();
                stack.push(*v);
            }
            ans
        };

        let right_min = {
            let mut ans = vec![None; n];
            let mut stack: Vec<i32> = vec![];
            for (i, v) in nums.iter().enumerate().rev() {
                while let Some(u) = stack.last() {
                    if u >= v {
                        stack.pop();
                    } else {
                        break;
                    }
                }
                ans[i] = stack.first().cloned();
                stack.push(*v);
            }
            ans
        };

        left_min
            .into_iter()
            .zip(right_min)
            .zip(nums)
            .filter(|((x, z), _y)| x.is_some() && z.is_some())
            .map(|((x, z), y)| x.unwrap() + y + z.unwrap())
            .min()
            .unwrap_or(-1)
    }
}

#[test]
fn test1() {
    let nums = vec![8, 6, 1, 5, 3];
    assert_eq!(Solution::minimum_sum(nums), 9);
}

#[test]
fn test2() {
    let nums = vec![5, 4, 8, 7, 10, 2];
    assert_eq!(Solution::minimum_sum(nums), 13);
}

#[test]
fn test3() {
    let nums = vec![6, 5, 4, 3, 4, 5];
    assert_eq!(Solution::minimum_sum(nums), -1);
}
