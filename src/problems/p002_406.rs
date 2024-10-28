use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

pub fn min_groups(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_by(|a, b| match a[0].cmp(&b[0]) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => a[1].cmp(&b[1]),
    });
    let mut heap = BinaryHeap::new();
    for v in &intervals {
        if let Some(Reverse(r)) = heap.peek() {
            if *r < v[0] {
                heap.pop();
            }
        }
        heap.push(Reverse(v[1]));
    }
    heap.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let v = [[5, 10], [6, 8], [1, 5], [2, 3], [1, 10]];
        let v: Vec<Vec<_>> = v.into_iter().map(|w| w.into_iter().collect()).collect();
        assert_eq!(min_groups(v), 3);
    }
}
