use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap = BinaryHeap::from_iter(nums.into_iter().map(Reverse));
        while heap.len() > k as usize {
            heap.pop();
        }
        KthLargest { k: k as usize, heap }
    }
    
    fn add(&mut self, val: i32) -> i32 {
        if self.heap.len() < self.k {
            self.heap.push(Reverse(val));
        } else if val > self.heap.peek().unwrap().0 {
            self.heap.push(Reverse(val));
            self.heap.pop();
        }
        // println!("{:#?}", self.heap);
        self.heap.peek().unwrap().0
    }
}

#[test]
fn example() {
    let v = vec![0];
    let k = 2;
    let mut sol = KthLargest::new(k, v);

    let ans = [-1, 1, -2, -4, 3].into_iter()
        .map(|val| sol.add(val))
        .collect::<Vec<_>>();
    assert_eq!(ans, vec![-1, 0, 0, 0, 1]);
}
