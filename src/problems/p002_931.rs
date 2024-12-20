pub struct Solution;

impl Solution {
    pub fn max_spending(values: Vec<Vec<i32>>) -> i64 {
        let n = values[0].len();

        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut heap: BinaryHeap<Reverse<(i32, &[i32], usize)>> = BinaryHeap::new();
        for row in values.iter() {
            heap.push(Reverse((row[n - 1], row, n - 1)));
        }

        let mut ans = 0i64;
        let mut day = 0i64;
        while let Some(Reverse((v, row, col))) = heap.pop() {
            day += 1;
            ans += v as i64 * day;
            if col > 0 {
                heap.push(Reverse((row[col - 1], row, col - 1)));
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let values = nested_vec![[8, 5, 2], [6, 4, 1], [9, 7, 3]];
        assert_eq!(Solution::max_spending(values), 285);
    }

    #[test]
    fn sample2() {
        let values = nested_vec![[10, 8, 6, 4, 2], [9, 7, 5, 3, 2]];
        assert_eq!(Solution::max_spending(values), 386);
    }
}
