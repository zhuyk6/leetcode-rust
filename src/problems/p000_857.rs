struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let n = quality.len();
        let k = k as usize;

        let mut indices: Vec<usize> = (0..n).collect();
        indices.sort_unstable_by(|&idx1, &idx2| {
            (quality[idx2] as i64 * wage[idx1] as i64)
                .cmp(&(quality[idx1] as i64 * wage[idx2] as i64))
        });

        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::new();
        let mut sum = 0;
        let mut ans = f64::MAX;
        for idx in indices {
            while heap.len() >= k {
                sum -= heap.pop().unwrap();
            }
            heap.push(quality[idx]);
            sum += quality[idx];

            if heap.len() == k {
                ans = ans.min(sum as f64 * wage[idx] as f64 / quality[idx] as f64);
            }
        }

        ans
    }
}

#[test]
fn test1() {
    let quality = vec![10, 20, 5];
    let wage = vec![70, 50, 30];
    let k = 2;
    assert!((Solution::mincost_to_hire_workers(quality, wage, k) - 105.0).abs() < 1e-5);
}

#[test]
fn test2() {
    let quality = vec![3, 1, 10, 10, 1];
    let wage = vec![4, 8, 2, 2, 7];
    let k = 3;
    assert!((Solution::mincost_to_hire_workers(quality, wage, k) - 30.66667).abs() < 1e-5);
}
