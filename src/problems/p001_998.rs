pub struct Solution;

struct JoinSet {
    fa: Vec<usize>,
}

impl JoinSet {
    fn new(n: usize) -> Self {
        Self {
            fa: (0..n).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.fa[x] != x {
            self.fa[x] = self.find(self.fa[x]);
        }
        self.fa[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let fx = self.find(x);
        let fy = self.find(y);
        if fx != fy {
            self.fa[fx] = fy;
        }
    }
}

const MAX_VAL: i32 = 100_000;

impl Solution {
    pub fn gcd_sort(nums: Vec<i32>) -> bool {
        let mut join_set = JoinSet::new((MAX_VAL + 1) as usize);

        for &x in &nums {
            let mut num = x;
            let mut factor = 2;
            while factor * factor <= num {
                if num % factor == 0 {
                    while num % factor == 0 {
                        num /= factor;
                    }
                    join_set.union(x as usize, factor as usize);
                }

                factor += 1;
            }
            if num > 1 {
                join_set.union(x as usize, num as usize);
            }
        }

        let mut sorted_nums = nums.clone();
        sorted_nums.sort_unstable();

        for (&x, &y) in nums.iter().zip(&sorted_nums) {
            if x != y && join_set.find(x as usize) != join_set.find(y as usize) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![7, 21, 3];
        let res = Solution::gcd_sort(nums);
        assert!(res);
    }

    #[test]
    fn sample2() {
        let nums = vec![5, 2, 6, 2];
        let res = Solution::gcd_sort(nums);
        assert!(!res);
    }

    #[test]
    fn sample3() {
        let nums = vec![10, 5, 9, 3, 15];
        let res = Solution::gcd_sort(nums);
        assert!(res);
    }
}
