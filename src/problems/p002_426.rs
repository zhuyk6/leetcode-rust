struct Solution;

const BOUND: i32 = 30_000;

struct BinaryIndexTree {
    n: usize,
    a: Vec<i32>,
}

impl BinaryIndexTree {
    fn new(n: usize) -> Self {
        BinaryIndexTree {
            n,
            a: vec![0; n + 1],
        }
    }

    #[inline]
    fn lowbit(x: usize) -> usize {
        x & ((!x) + 1)
    }

    fn add(&mut self, mut x: usize, delta: i32) {
        while x <= self.n {
            self.a[x] += delta;
            x += BinaryIndexTree::lowbit(x);
        }
    }

    fn query(&self, mut x: usize) -> i32 {
        let mut acc = 0;
        while x > 0 {
            acc += self.a[x];
            x -= BinaryIndexTree::lowbit(x);
        }
        acc
    }

    // fn sum(&self, l: usize, r: usize) -> i32 {
    //     assert!(l > 0);
    //     assert!(r >= l);

    //     self.query(r) - self.query(l - 1)
    // }
}

#[allow(unused)]
impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64 {
        let arr: Vec<i32> = nums1.into_iter().zip(nums2).map(|(x, y)| x - y).collect();

        const SHIFT: i32 = BOUND + 1;
        let mut tree = BinaryIndexTree::new((SHIFT + BOUND + 1) as usize);

        let mut ans = 0;
        for v in arr {
            let u = v + diff;
            ans += tree.query((SHIFT + u) as usize) as i64;
            tree.add((SHIFT + v) as usize, 1);
        }

        ans
    }
}

#[test]
fn test1() {
    let nums1 = vec![3, 2, 5];
    let nums2 = vec![2, 2, 1];
    let diff = 1;
    assert_eq!(Solution::number_of_pairs(nums1, nums2, diff), 3);
}

#[test]
fn test2() {
    let nums1 = vec![3, -1];
    let nums2 = vec![-2, 2];
    let diff = -1;
    assert_eq!(Solution::number_of_pairs(nums1, nums2, diff), 0);
}
