pub struct Solution;

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let n = costs.len();
        let candidates = candidates as usize;

        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::new();

        let mut l = 0;
        while l < candidates {
            heap.push(Reverse((costs[l], l)));
            l += 1;
        }

        let mut r = n - 1;
        while r >= l && n - r <= candidates {
            heap.push(Reverse((costs[r], r)));
            r -= 1;
        }

        let mut ans: i64 = 0;
        for _ in 0..k {
            let Reverse((c, idx)) = heap.pop().unwrap();
            ans += c as i64;
            if idx < l && l <= r {
                heap.push(Reverse((costs[l], l)));
                l += 1;
            }
            if idx > r && l <= r {
                heap.push(Reverse((costs[r], r)));
                r -= 1;
            }
        }

        ans
    }
}

#[test]
fn test1() {
    let costs = vec![17, 12, 10, 2, 7, 2, 11, 20, 8];
    let k = 3;
    let candidates = 4;
    assert_eq!(Solution::total_cost(costs, k, candidates), 11);
}

#[test]
fn test2() {
    let costs = vec![1, 2, 4, 1];
    let k = 3;
    let candidates = 3;
    assert_eq!(Solution::total_cost(costs, k, candidates), 4);
}
