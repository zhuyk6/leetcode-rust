pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn second_greater_element_heap(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut heap0: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        let mut heap1: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        let mut answer = vec![-1; n];

        for (i, &v) in nums.iter().enumerate() {
            while !heap1.is_empty() && -v < heap1.peek().unwrap().0 {
                let (_, idx) = heap1.pop().unwrap();
                println!("idx = {idx}");
                answer[idx] = nums[i];
            }
            while !heap0.is_empty() && -v < heap0.peek().unwrap().0 {
                let item = heap0.pop().unwrap();
                println!("item = {item:?}");
                heap1.push(item);
            }
            heap0.push((-v, i));
        }

        answer
    }

    pub fn second_greater_element(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut stk0: Vec<usize> = vec![usize::MAX; n];
        let mut stk1: Vec<usize> = vec![usize::MAX; n];
        let mut tmp: Vec<usize> = vec![usize::MAX; n];

        let mut answer = vec![-1; n];

        let mut top0 = 0;
        let mut top1 = 0;

        for (i, &v) in nums.iter().enumerate() {
            while top1 > 0 && nums[stk1[top1 - 1]] < v {
                answer[stk1[top1 - 1]] = nums[i];
                top1 -= 1;
            }
            let mut tmp_size = 0;
            while top0 > 0 && nums[stk0[top0 - 1]] < v {
                tmp[tmp_size] = stk0[top0 - 1];
                tmp_size += 1;
                top0 -= 1;
            }
            while tmp_size > 0 {
                stk1[top1] = tmp[tmp_size - 1];
                top1 += 1;
                tmp_size -= 1;
            }
            stk0[top0] = i;
            top0 += 1;
        }

        answer
    }
}

#[test]
fn test1() {
    let nums = vec![2, 4, 0, 9, 6];
    assert_eq!(
        Solution::second_greater_element(nums),
        vec![9, 6, 6, -1, -1]
    );
}

#[test]
fn test2() {
    let nums = vec![3, 3];
    assert_eq!(Solution::second_greater_element(nums), vec![-1, -1]);
}
