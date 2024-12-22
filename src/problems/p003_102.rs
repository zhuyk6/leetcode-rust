pub struct Solution;

impl Solution {
    pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();

        let (mut arr_p, mut arr_q) = points.into_iter().enumerate().fold(
            (vec![], vec![]),
            |(mut arr_p, mut arr_q), (i, point)| {
                arr_p.push((point[0] + point[1], i));
                arr_q.push((point[0] - point[1], i));
                (arr_p, arr_q)
            },
        );

        arr_p.sort_unstable();
        arr_q.sort_unstable();

        dbg!(&arr_p);
        dbg!(&arr_q);

        fn remove(arr_p: &[(i32, usize)], arr_q: &[(i32, usize)], remove_idx: usize) -> i32 {
            fn remove_(arr: &[(i32, usize)], remove_idx: usize) -> i32 {
                let n = arr.len();
                let min = if arr[0].1 == remove_idx {
                    arr[1].0
                } else {
                    arr[0].0
                };
                let max = if arr[n - 1].1 == remove_idx {
                    arr[n - 2].0
                } else {
                    arr[n - 1].0
                };
                max - min
            }

            remove_(arr_p, remove_idx).max(remove_(arr_q, remove_idx))
        }

        (0..n).map(|i| remove(&arr_p, &arr_q, i)).min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let points = nested_vec![[3, 10], [5, 15], [10, 2], [4, 4]];
        assert_eq!(Solution::minimum_distance(points), 12)
    }

    #[test]
    fn sample2() {
        let points = nested_vec![[1, 1], [1, 1], [1, 1]];
        assert_eq!(Solution::minimum_distance(points), 0)
    }
}
