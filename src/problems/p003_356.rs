pub struct Solution;

pub mod segment_tree {
    pub trait Node {
        fn max_val(&self) -> i32;
        fn update_node(&mut self, val_delta: i32);
        fn update_interval(&mut self, left: usize, right: usize, delta: i32);
    }

    struct Leaf {
        val: i32,
        pos: usize,
    }

    impl Node for Leaf {
        fn max_val(&self) -> i32 {
            self.val
        }

        fn update_interval(&mut self, left: usize, right: usize, delta: i32) {
            if self.pos >= left && self.pos <= right {
                self.update_node(delta);
            }
        }

        fn update_node(&mut self, val_delta: i32) {
            self.val += val_delta;
        }
    }

    struct InternalNode {
        left: usize,
        right: usize,
        lc: Box<dyn Node>,
        rc: Box<dyn Node>,
        max_val: i32,
        delta: Option<i32>,
    }

    impl InternalNode {
        fn push_down(&mut self) {
            if let Some(delta) = self.delta.take() {
                self.lc.update_node(delta);
                self.rc.update_node(delta);
            }
        }

        fn push_up(&mut self) {
            self.max_val = std::cmp::max(self.lc.max_val(), self.rc.max_val());
        }
    }

    impl Node for InternalNode {
        fn max_val(&self) -> i32 {
            self.max_val
        }

        fn update_interval(&mut self, left: usize, right: usize, delta: i32) {
            if left <= self.left && self.right <= right {
                self.update_node(delta);
            } else {
                self.push_down();
                let mid = (self.left + self.right) / 2;
                if left <= mid {
                    self.lc.update_interval(left, right, delta);
                }
                if mid < right {
                    self.rc.update_interval(left, right, delta);
                }
                self.push_up();
            }
        }

        fn update_node(&mut self, val_delta: i32) {
            self.max_val += val_delta;
            *self.delta.get_or_insert_default() += val_delta;
        }
    }

    pub fn build(nums: &[i32]) -> Box<dyn Node> {
        fn _build(nums: &[i32], left: usize, right: usize) -> Box<dyn Node> {
            if left == right {
                return Box::new(Leaf {
                    val: nums[left],
                    pos: left,
                });
            }
            let mid = (left + right) / 2;
            let lc = _build(nums, left, mid);
            let rc = _build(nums, mid + 1, right);
            let max_val = std::cmp::max(lc.max_val(), rc.max_val());
            Box::new(InternalNode {
                left,
                right,
                lc,
                rc,
                max_val,
                delta: None,
            })
        }

        _build(nums, 0, nums.len() - 1)
    }
}

impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();

        let mut diff = vec![0; n];
        diff[0] = nums[0];
        for i in 1..n {
            diff[i] = nums[i] - nums[i - 1];
        }

        let mut acc = 0;
        let mut q_id = 0usize;
        for i in 0..n {
            acc += diff[i];
            while acc > 0 && q_id < queries.len() {
                let (l, r, d) = (
                    queries[q_id][0] as usize,
                    queries[q_id][1] as usize,
                    queries[q_id][2],
                );
                diff[l] -= d;
                if r + 1 < n {
                    diff[r + 1] += d;
                }
                if l <= i && i <= r {
                    acc -= d;
                }
                q_id += 1;
            }
            if acc > 0 {
                return -1;
            }
        }

        q_id as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![2, 0, 2];
        let queries = [[0, 2, 1], [0, 2, 1], [1, 1, 3]];
        let queries: Vec<Vec<i32>> = queries.iter().map(|x| x.to_vec()).collect();
        assert_eq!(Solution::min_zero_array(nums, queries), 2);
    }

    #[test]
    fn sample2() {
        let nums = vec![4, 3, 2, 1];
        let queries = [[1, 3, 2], [0, 2, 1]];
        let queries: Vec<Vec<i32>> = queries.iter().map(|x| x.to_vec()).collect();
        assert_eq!(Solution::min_zero_array(nums, queries), -1);
    }
}
