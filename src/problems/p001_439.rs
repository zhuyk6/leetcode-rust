pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    fn kth_small_between2(a: Vec<i32>, b: Vec<i32>, mut k: usize) -> Vec<i32> {
        let mut heap: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
        let mut c = Vec::new();

        let la = a.len();
        let _lb = b.len();
        for (j, bj) in b.iter().enumerate() {
            heap.push((Reverse(a[0] + bj), 0, j));
        }
        while let Some((Reverse(sum), i, j)) = heap.pop() {
            c.push(sum);
            k -= 1;
            if k == 0 {
                break;
            }

            if i + 1 < la {
                heap.push((Reverse(a[i + 1] + b[j]), i + 1, j));
            }
        }
        c
    }

    let k = k as usize;
    *mat.into_iter()
        .reduce(|a, b| kth_small_between2(a, b, k))
        .unwrap()
        .get(k - 1)
        .unwrap()
}

#[test]
fn example() {
    let mat = vec![vec![1, 3, 11], vec![2, 4, 6]];
    let k = 5;
    assert_eq!(kth_smallest(mat, k), 7);
}
