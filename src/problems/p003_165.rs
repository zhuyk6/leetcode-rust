pub struct Solution;

trait Node {
    fn sum(&self, x: usize) -> i64;
    fn max_sum(&self) -> i64;
    fn modify(&mut self, pos: usize, new_val: i64);
}

#[derive(Debug)]
struct Leaf {
    #[allow(unused)]
    pos: usize,
    val: i64,
}

struct Interval {
    left: usize,
    right: usize,
    lc: Box<dyn Node>,
    rc: Box<dyn Node>,
    /// 00 -- select none;
    /// 10 -- select left;
    /// 01 -- select right;
    /// 11 -- select both;
    max_sum: [i64; 4],
}

impl Node for Leaf {
    fn max_sum(&self) -> i64 {
        0.max(self.val)
    }

    fn modify(&mut self, _pos: usize, new_val: i64) {
        self.val = new_val;
    }

    fn sum(&self, x: usize) -> i64 {
        if x == 3 { self.val } else { 0 }
    }
}

impl Node for Interval {
    fn max_sum(&self) -> i64 {
        self.max_sum.iter().copied().max().unwrap()
    }

    fn modify(&mut self, pos: usize, new_val: i64) {
        let mid = (self.left + self.right) >> 1;
        if pos <= mid {
            self.lc.modify(pos, new_val);
        } else {
            self.rc.modify(pos, new_val);
        }
        self.update();
    }

    fn sum(&self, x: usize) -> i64 {
        self.max_sum[x]
    }
}

impl Interval {
    fn update(&mut self) {
        for x in 0..4 {
            self.max_sum[x] = i64::MIN;

            for y in 0..4 {
                for z in 0..4 {
                    if x & 0b01 == z & 0b01
                        && x & 0b10 == y & 0b10
                        && (y & 0b01 > 0) != (z & 0b10 > 0)
                    {
                        self.max_sum[x] = self.max_sum[x].max(self.lc.sum(y) + self.rc.sum(z));
                    }
                }
            }
        }
    }
}

struct SegmentTree {
    root: Box<dyn Node>,
}

impl SegmentTree {
    fn build(nums: Vec<i32>) -> Self {
        fn build(left: usize, right: usize, nums: &[i32]) -> Box<dyn Node> {
            match left.cmp(&right) {
                std::cmp::Ordering::Less => {
                    let mid = (left + right) / 2;
                    let lc = build(left, mid, nums);
                    let rc = build(mid + 1, right, nums);
                    let mut node = Interval {
                        left,
                        right,
                        lc,
                        rc,
                        max_sum: [0; 4],
                    };
                    node.update();
                    Box::new(node)
                }
                std::cmp::Ordering::Equal => {
                    let node = Leaf {
                        pos: left,
                        val: nums[left] as i64,
                    };
                    Box::new(node)
                }
                std::cmp::Ordering::Greater => panic!("error!"),
            }
        }

        Self {
            root: build(0, nums.len() - 1, &nums),
        }
    }

    #[inline]
    fn query(&self) -> i64 {
        self.root.max_sum()
    }

    fn modify(&mut self, pos: usize, new_val: i64) {
        self.root.modify(pos, new_val);
    }
}

impl Solution {
    pub fn maximum_sum_subsequence(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut tree = SegmentTree::build(nums);
        let mut ans = 0;
        const MOD: i64 = 1_000_000_007;
        for query in queries {
            tree.modify(query[0] as usize, query[1] as i64);
            // eprintln!("query {}", tree.query());
            ans += tree.query();
        }
        (ans % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::nested_vec;

    #[test]
    fn sample1() {
        let nums = vec![3, 5, 9];
        let queries = nested_vec![[1, -2], [0, -3]];
        assert_eq!(Solution::maximum_sum_subsequence(nums, queries), 21);
    }

    #[test]
    fn sample2() {
        let nums = vec![0, -1];
        let queries = nested_vec![[0, -5]];
        assert_eq!(Solution::maximum_sum_subsequence(nums, queries), 0);
    }

    #[test]
    fn issue1() {
        let nums = vec![0, 3, 3, 3, 1, -2];
        let queries = nested_vec![[4, 0], [1, 0]];
        assert_eq!(Solution::maximum_sum_subsequence(nums, queries), 9);
    }
}
