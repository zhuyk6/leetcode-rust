struct Solution;

struct BinaryIndexedTree {
    len: usize,
    v: Vec<usize>,
}

impl BinaryIndexedTree {
    /// manage [1, n]
    fn new(n: usize) -> Self {
        BinaryIndexedTree {
            len: n,
            v: vec![0; n + 1],
        }
    }

    #[inline]
    fn lowbit(x: usize) -> usize {
        x & (!x + 1)
    }

    fn add(&mut self, mut x: usize, d: usize) {
        while x <= self.len {
            self.v[x] += d;
            x += BinaryIndexedTree::lowbit(x);
        }
    }

    fn sum(&self, mut x: usize) -> usize {
        let mut acc = 0;
        while x > 0 {
            acc += self.v[x];
            x -= BinaryIndexedTree::lowbit(x);
        }
        acc
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ordered_nums = nums.clone();
        ordered_nums.sort_unstable();
        ordered_nums.dedup();
        let n = ordered_nums.len();

        let get_order = |v: i32| -> usize {
            ordered_nums.partition_point(|u| *u <= v)
        };

        let mut set1 = BinaryIndexedTree::new(n);
        let mut set2 = BinaryIndexedTree::new(n);
        let mut arr1 = vec![];
        let mut arr2 = vec![];

        set1.add(get_order(nums[0]), 1);
        set2.add(get_order(nums[1]), 1);
        arr1.push(nums[0]);
        arr2.push(nums[1]);

        for &v in &nums[2..] {
            let p = get_order(v);
            let cnt1 = arr1.len() - set1.sum(p);
            let cnt2 = arr2.len() - set2.sum(p);

            match cnt1.cmp(&cnt2) {
                std::cmp::Ordering::Greater => {
                    set1.add(p, 1);
                    arr1.push(v);
                }
                std::cmp::Ordering::Less => {
                    set2.add(p, 1);
                    arr2.push(v);
                }
                std::cmp::Ordering::Equal => {
                    if arr1.len() <= arr2.len() {
                        set1.add(p, 1);
                        arr1.push(v);
                    } else {
                        set2.add(p, 1);
                        arr2.push(v);
                    }
                }
            }
        }

        arr1.into_iter().chain(arr2).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![2, 1, 3, 3];
        let ans = vec![2, 3, 1, 3];
        assert_eq!(Solution::result_array(nums), ans);
    }

    #[test]
    fn sample2() {
        let nums = vec![5, 14, 3, 1, 2];
        let ans = vec![5, 3, 1, 2, 14];
        assert_eq!(Solution::result_array(nums), ans);
    }
}
