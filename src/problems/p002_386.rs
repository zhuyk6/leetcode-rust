struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused)]
impl Solution {
    pub fn k_sum(mut nums: Vec<i32>, mut k: i32) -> i64 {
        let sum_pos: i64 = nums.iter().filter(|v| **v > 0).map(|v| *v as i64).sum();

        if k == 1 {
            return sum_pos;
        }

        for v in &mut nums {
            *v = v.abs();
        }

        let n = nums.len();
        nums.sort_unstable();

        let mut heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
        heap.push(Reverse((nums[0] as i64, 0)));

        let mut choose = 0;
        while k > 1 {
            if let Some(Reverse((v, i))) = heap.pop() {
                choose = v;
                if i + 1 < n {
                    heap.push(Reverse((v - nums[i] as i64 + nums[i + 1] as i64, i + 1)));
                    heap.push(Reverse((v + nums[i + 1] as i64, i + 1)));
                }
            }
            k -= 1;
        }
        sum_pos - choose
    }
}

#[test]
fn test1() {
    let nums = vec![2, 4, -2];
    let k = 1;
    assert_eq!(Solution::k_sum(nums, k), 6);
}

#[test]
fn test2() {
    let nums = vec![1, -2, 3, 4, -10, 12];
    let k = 16;
    assert_eq!(Solution::k_sum(nums, k), 10);
}
