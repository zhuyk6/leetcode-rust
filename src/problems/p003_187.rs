pub struct Solution;

struct BinaryIndexTree {
    n: usize,
    a: Vec<i32>,
}

impl BinaryIndexTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            a: vec![0; n + 1],
        }
    }

    fn add(&mut self, mut i: usize, delta: i32) {
        while i <= self.n {
            self.a[i] += delta;
            i += i & i.wrapping_neg();
        }
    }

    fn sum(&self, mut i: usize) -> i32 {
        let mut sum = 0;
        while i > 0 {
            sum += self.a[i];
            i -= i & i.wrapping_neg();
        }
        sum
    }
}

impl Solution {
    pub fn count_of_peaks(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut bit = BinaryIndexTree::new(n);
        for i in 1..n - 1 {
            if nums[i] > nums[i - 1] && nums[i] > nums[i + 1] {
                bit.add(i + 1, 1);
            }
        }
        let mut ans = vec![];
        for query in queries {
            let ty = query[0];
            match ty {
                1 => {
                    let l = query[1] as usize;
                    let r = query[2] as usize;
                    if r > 0 && l < r - 1 {
                        ans.push(bit.sum(r) - bit.sum(l + 1));
                    } else {
                        ans.push(0);
                    }
                }
                2 => {
                    fn update(nums: &[i32], i: usize, bit: &mut BinaryIndexTree, delta: i32) {
                        let n = nums.len();
                        if i > 0 && i < n - 1 {
                            let prev = nums[i - 1];
                            let next = nums[i + 1];
                            if prev < nums[i] && nums[i] > next {
                                bit.add(i + 1, delta);
                            }
                        }
                    }

                    let i = query[1] as usize;
                    let v = query[2];

                    for j in i.saturating_sub(1)..=(i + 1).min(n - 1) {
                        update(&nums, j, &mut bit, -1);
                    }
                    nums[i] = v;
                    for j in i.saturating_sub(1)..=(i + 1).min(n - 1) {
                        update(&nums, j, &mut bit, 1);
                    }
                }
                _ => panic!("invalid input"),
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
    fn test_lowbit() {
        fn lowbit1(x: usize) -> usize {
            x & x.wrapping_neg()
        }

        fn lowbit2(x: usize) -> usize {
            x & (!x + 1)
        }

        for i in 1..=100 {
            assert_eq!(lowbit1(i), lowbit2(i));
            eprintln!("i = {:10b}, {:b}", i, i.wrapping_neg())
        }
    }

    #[test]
    fn sample1() {
        let nums = vec![3, 1, 4, 2, 5];
        let queries = nested_vec![[2, 3, 4], [1, 0, 4]];
        assert_eq!(Solution::count_of_peaks(nums, queries), vec![0])
    }

    #[test]
    fn sample2() {
        let nums = vec![4, 1, 4, 2, 1, 5];
        let queries = nested_vec![[2, 2, 4], [1, 0, 2], [1, 0, 4]];
        assert_eq!(Solution::count_of_peaks(nums, queries), vec![0, 1])
    }
}
