pub struct Solution;

impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let n = apples.len();
        let mut heap = std::collections::BinaryHeap::new();
        use std::cmp::Reverse;

        let mut ans = 0;
        for (i, (&num, &day)) in apples.iter().zip(days.iter()).enumerate() {
            if num > 0 && day > 0 {
                heap.push(Reverse((day as usize + i - 1, num)));
            }

            while let Some(Reverse((day, num))) = heap.pop() {
                if day >= i {
                    heap.push(Reverse((day, num)));
                    break;
                }
            }
            if let Some(Reverse((day, mut num))) = heap.pop() {
                ans += 1;
                num -= 1;
                if num > 0 {
                    heap.push(Reverse((day, num)));
                }
            }
        }
        let mut cur = n;
        while let Some(Reverse((day, num))) = heap.pop() {
            if day >= cur {
                let tmp = (day - cur + 1).min(num as usize);
                ans += tmp;
                cur += tmp;
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let apples = vec![1, 2, 3, 5, 2];
        let days = vec![3, 2, 1, 4, 2];
        assert_eq!(Solution::eaten_apples(apples, days), 7)
    }

    #[test]
    fn sample2() {
        let apples = vec![3, 0, 0, 0, 0, 2];
        let days = vec![3, 0, 0, 0, 0, 2];
        assert_eq!(Solution::eaten_apples(apples, days), 5)
    }
}
