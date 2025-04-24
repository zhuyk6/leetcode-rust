pub struct Solution;

#[allow(clippy::upper_case_acronyms)]
struct BIT {
    n: usize,
    arr: Vec<u32>,
}

impl BIT {
    fn new(n: usize) -> Self {
        BIT {
            n,
            arr: vec![0; n + 1],
        }
    }

    #[inline(always)]
    fn lowbit(x: usize) -> usize {
        x & (!x + 1)
    }

    fn add(&mut self, idx: usize, val: u32) {
        let mut idx = idx + 1;
        while idx <= self.n {
            self.arr[idx] += val;
            idx += BIT::lowbit(idx);
        }
    }

    fn sum(&self, idx: usize) -> u32 {
        let mut idx = idx + 1;
        let mut res = 0;
        while idx > 0 {
            res += self.arr[idx];
            idx -= BIT::lowbit(idx);
        }
        res
    }
}

impl Solution {
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let n = nums1.len();

        let mut pos2: Vec<usize> = vec![0; n];
        for (i, &v) in nums2.iter().enumerate() {
            pos2[v as usize] = i;
        }

        let mut left_cnt: Vec<u32> = vec![0; n];
        let mut set = BIT::new(n);
        for (i, &v) in nums1.iter().enumerate() {
            let j = pos2[v as usize];

            left_cnt[i] = set.sum(j);
            set.add(j, 1);
        }

        let mut right_cnt: Vec<u32> = vec![0; n];
        let mut set = BIT::new(n);
        for (i, &v) in nums1.iter().enumerate().rev() {
            let j = pos2[v as usize];

            right_cnt[i] = (n - i - 1) as u32 - set.sum(j);
            set.add(j, 1);
        }

        left_cnt
            .into_iter()
            .zip(right_cnt)
            .map(|(c1, c2)| c1 as i64 * c2 as i64)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums1 = vec![2, 0, 1, 3];
        let nums2 = vec![0, 1, 2, 3];
        assert_eq!(Solution::good_triplets(nums1, nums2), 1);
    }

    #[test]
    fn sample2() {
        let nums1 = vec![4, 0, 1, 3, 2];
        let nums2 = vec![4, 1, 0, 2, 3];
        assert_eq!(Solution::good_triplets(nums1, nums2), 4);
    }
}
