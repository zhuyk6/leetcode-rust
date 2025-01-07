pub struct Solution;

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>, target: Vec<i32>) -> i64 {
        let arr: Vec<i32> = target.into_iter().zip(nums).map(|(t, s)| t - s).collect();

        let mut ans = 0i64;
        let mut sign = arr[0].signum();
        let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
        heap.push(Reverse((arr[0].abs(), 0)));

        fn calc(mut heap: BinaryHeap<Reverse<(i32, usize)>>) -> i64 {
            let mut ret = 0i64;
            let mut map: BTreeMap<usize, i32> = BTreeMap::new();
            while let Some(Reverse((v, i))) = heap.pop() {
                let left = map.range(..i).next_back().map(|(_, val)| *val);
                let right = map.range(i..).next().map(|(_, val)| *val);

                if let Some(val) = left.max(right) {
                    ret += (v - val) as i64;
                } else {
                    ret += v as i64;
                }
                map.insert(i, v);
            }
            ret
        }

        for (i, &v) in arr.iter().enumerate().skip(1) {
            if v.signum() != sign {
                ans += calc(std::mem::take(&mut heap));
                sign = v.signum();
                heap.push(Reverse((v.abs(), i)));
            } else {
                heap.push(Reverse((v.abs(), i)));
            }
        }
        ans += calc(std::mem::take(&mut heap));
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![3, 5, 1, 2];
        let target = vec![4, 6, 2, 4];
        assert_eq!(Solution::minimum_operations(nums, target), 2);
    }

    #[test]
    fn sample2() {
        let nums = vec![1, 3, 2];
        let target = vec![2, 1, 4];
        assert_eq!(Solution::minimum_operations(nums, target), 5);
    }

    #[test]
    fn issue() {
        let nums = vec![9, 2, 6, 10, 4, 8, 3, 4, 2, 3];
        let target = vec![9, 5, 5, 1, 7, 9, 8, 7, 6, 5];
        assert_eq!(Solution::minimum_operations(nums, target), 20);
    }

    #[test]
    fn issue2() {
        let nums = vec![2, 7, 10, 7, 8, 3, 5];
        let target = vec![1, 7, 4, 1, 1, 9, 2];
        assert_eq!(Solution::minimum_operations(nums, target), 17);
    }

    #[test]
    fn issue3() {
        let nums = vec![1, 2, 1];
        let target = vec![3, 3, 4];
        assert_eq!(Solution::minimum_operations(nums, target), 4);
    }
}
