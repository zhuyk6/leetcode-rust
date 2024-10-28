pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut sum: i64 = gifts.iter().map(|x| *x as i64).sum();
        let mut heap = BinaryHeap::from_iter(gifts.into_iter().map(|x| x as i64));
        for _ in 0..k {
            let x = heap.pop().unwrap();
            let y = (x as f64).sqrt().floor() as i64;
            sum -= x - y;
            heap.push(y);
        }

        sum
    }
}

#[test]
fn test1() {
    let gifts = vec![25, 64, 9, 4, 100];
    let k = 4;
    assert_eq!(Solution::pick_gifts(gifts, k), 29);
}
