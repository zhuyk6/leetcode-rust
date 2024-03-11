struct Solution;

use std::collections::HashMap;

#[allow(unused)]
impl Solution {
    pub fn minimum_seconds(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let f = |pos: &[usize]| -> usize {
            pos.windows(2)
                .map(|w| (w[1] - w[0]) / 2)
                .max()
                .map_or(n / 2, |v| v.max((pos[0] + n - *pos.last().unwrap()) / 2))
        };

        let mut pos_of_val: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, v) in nums.into_iter().enumerate() {
            pos_of_val.entry(v).or_default().push(i);
        }

        for (k, v) in &pos_of_val {
            println!("k = {}, f = {}", k, f(v));
        }

        pos_of_val.into_values().map(|pos| f(&pos)).min().unwrap() as i32
    }
}

#[test]
fn test1() {
    let nums = vec![1, 2, 1, 2];
    assert_eq!(Solution::minimum_seconds(nums), 1);
}

#[test]
fn test2() {
    let nums = vec![2, 1, 3, 3, 2];
    assert_eq!(Solution::minimum_seconds(nums), 2);
}

#[test]
fn test3() {
    let nums = vec![5, 5, 5, 5];
    assert_eq!(Solution::minimum_seconds(nums), 0);
}

#[test]
fn test4() {
    let nums = vec![1, 2, 1, 3];
    assert_eq!(Solution::minimum_seconds(nums), 1);
}
