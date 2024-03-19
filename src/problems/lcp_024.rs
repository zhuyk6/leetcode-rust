struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

const MOD: i64 = 1_000_000_007i64;

#[allow(unused)]
impl Solution {
    pub fn nums_game(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        for (i, v) in nums.iter_mut().enumerate() {
            *v -= i as i32;
        }

        let mut heap1: BinaryHeap<i32> = BinaryHeap::new();
        let mut heap2: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let mut sum1 = 0i64;
        let mut sum2 = 0i64;

        let mut ans: Vec<i32> = Vec::with_capacity(n);

        for (i, v) in nums.into_iter().enumerate() {
            if (heap1.len() + heap2.len()) % 2 == 0 {
                // even
                if heap1.is_empty() && heap2.is_empty() {
                    heap1.push(v);
                    sum1 += v as i64;
                } else {
                    let &x = heap1.peek().unwrap();
                    let &Reverse(y) = heap2.peek().unwrap();
                    if v <= y {
                        // push v to heap1
                        heap1.push(v);
                        sum1 += v as i64;
                    } else {
                        // move y to heap1
                        heap2.pop();
                        sum2 -= y as i64;
                        heap1.push(y);
                        sum1 += y as i64;
                        // push v to heap2
                        heap2.push(Reverse(v));
                        sum2 += v as i64;
                    }
                }
                let m = heap1.len();
                let &vm = heap1.peek().unwrap();
                let tmp = (2 * m as i64 - (i + 1) as i64) * vm as i64;
                ans.push(((tmp - sum1 + sum2) % MOD) as i32);
            } else {
                // odd
                // get the middle number
                let x = if heap1.len() > heap2.len() {
                    let x = heap1.pop().unwrap();
                    sum1 -= x as i64;
                    x
                } else {
                    let Reverse(x) = heap2.pop().unwrap();
                    sum2 -= x as i64;
                    x
                };
                // push v
                if x >= v {
                    heap1.push(v);
                    sum1 += v as i64;
                    heap2.push(Reverse(x));
                    sum2 += x as i64;
                } else {
                    heap2.push(Reverse(v));
                    sum2 += v as i64;
                    heap1.push(x);
                    sum1 += x as i64;
                }
                let m = heap1.len();
                let &vm = heap1.peek().unwrap();
                let tmp = (2 * m as i64 - (i + 1) as i64) * vm as i64;
                ans.push(((tmp - sum1 + sum2) % MOD) as i32);
            }
        }

        ans
    }
}

#[test]
fn test1() {
    let nums = vec![3, 4, 5, 1, 6, 7];
    let ans = vec![0, 0, 0, 5, 6, 7];
    assert_eq!(Solution::nums_game(nums), ans);
}

#[test]
fn test2() {
    let nums = vec![1, 2, 3, 4, 5];
    let ans = vec![0, 0, 0, 0, 0];
    assert_eq!(Solution::nums_game(nums), ans);
}

#[test]
fn test3() {
    let nums = vec![1, 1, 1, 2, 3, 4];
    let ans = vec![0, 1, 2, 3, 3, 3];
    assert_eq!(Solution::nums_game(nums), ans);
}

#[test]
fn test4() {
    let nums = vec![
        471, 626, 848, 957, 788, 138, 982, 721, 729, 956, 432, 1000, 478, 353, 586, 544, 304, 811,
        359, 535, 992, 684, 606, 39, 384, 799, 120, 608, 540, 914, 19, 62, 410, 711, 449, 370, 263,
        890, 124, 516, 861, 936, 729, 595, 768, 929, 649, 803, 65, 692, 548, 620, 871, 785, 15,
        629, 251, 764, 973, 484, 172, 923, 859, 29, 297, 215, 436, 74, 762, 885, 272, 504, 307,
        998, 83, 329, 520, 889, 584, 496, 443, 802, 180, 322, 640, 290, 292, 403, 11, 230, 743,
        183, 600, 741, 558, 780, 970, 777, 288,
    ];
    let ans = vec![
        0, 154, 375, 704, 704, 1355, 1547, 1617, 1617, 1843, 2142, 2410, 2665, 3046, 3188, 3373,
        3710, 3879, 4163, 4272, 4619, 4657, 4698, 5307, 5531, 5721, 6211, 6214, 6283, 6587, 7179,
        7729, 7923, 8029, 8186, 8423, 8725, 9049, 9492, 9544, 9836, 10202, 10317, 10337, 10489,
        10801, 10823, 10998, 11562, 11624, 11707, 11719, 11957, 12108, 12728, 12735, 13114, 13247,
        13581, 13737, 14199, 14487, 14703, 15318, 15659, 16083, 16285, 16850, 16972, 17216, 17586,
        17725, 18059, 18415, 18975, 19290, 19398, 19658, 19704, 19839, 20005, 20197, 20628, 20918,
        20945, 21269, 21579, 21779, 22368, 22739, 22880, 23300, 23300, 23440, 23484, 23661, 24023,
        24191, 24513,
    ];
    assert_eq!(Solution::nums_game(nums), ans);
}

#[test]
fn test5() {
    let nums = vec![471, 626, 848, 957, 788];
    let ans = vec![0, 154, 375, 704, 704];
    assert_eq!(Solution::nums_game(nums), ans);
}
