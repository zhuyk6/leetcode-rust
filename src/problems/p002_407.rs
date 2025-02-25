pub struct Solution;

#[derive(Debug, Clone)]
enum Node {
    Leaf {
        val: i32,
    },
    Inner {
        max: i32,
        left: i32,
        right: i32,
        lc: Box<Node>,
        rc: Box<Node>,
    },
}

impl Node {
    fn build(left: i32, right: i32) -> Box<Self> {
        match left.cmp(&right) {
            std::cmp::Ordering::Less => {
                let mid = (left + right) / 2;
                let lc = Node::build(left, mid);
                let rc = Node::build(mid + 1, right);
                Box::new(Node::Inner {
                    max: 0,
                    left,
                    right,
                    lc,
                    rc,
                })
            }
            std::cmp::Ordering::Equal => Box::new(Node::Leaf { val: 0 }),
            std::cmp::Ordering::Greater => panic!("left > right"),
        }
    }

    fn max(&self) -> i32 {
        match self {
            Node::Leaf { val } => *val,
            Node::Inner { max, .. } => *max,
        }
    }

    fn update(&mut self, idx: i32, val: i32) {
        match self {
            Node::Leaf { val: v } => *v = val,
            &mut Node::Inner {
                ref mut max,
                ref left,
                ref right,
                ref mut lc,
                ref mut rc,
            } => {
                let mid = (left + right) >> 1;
                if idx <= mid {
                    lc.update(idx, val);
                } else {
                    rc.update(idx, val);
                }
                *max = lc.max().max(rc.max());
            }
        }
    }

    fn query(&self, l: i32, r: i32) -> i32 {
        match self {
            Node::Leaf { val } => *val,
            Node::Inner {
                max,
                left,
                right,
                lc,
                rc,
            } => {
                if (l..=r).contains(left) && (l..=r).contains(right) {
                    return *max;
                }
                let mid = (left + right) >> 1;
                let mut ret = i32::MIN;
                if l <= mid {
                    ret = ret.max(lc.query(l, r));
                }
                if mid < r {
                    ret = ret.max(rc.query(l, r));
                }
                ret
            }
        }
    }
}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>, k: i32) -> i32 {
        let min_v = nums.iter().copied().min().unwrap();
        let max_v = nums.iter().copied().max().unwrap();
        let mut root = Node::build(min_v - 1, max_v);

        let mut ans = 0;
        for v in nums {
            let l = (v - k).max(min_v - 1);
            let r = v - 1;
            let val = root.query(l, r) + 1;
            ans = ans.max(val);
            root.update(v, val);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let nums = vec![4, 2, 1, 4, 3, 4, 5, 8, 15];
        let k = 3;
        assert_eq!(Solution::length_of_lis(nums, k), 5)
    }

    #[test]
    fn sample2() {
        let nums = vec![7, 4, 5, 1, 8, 12, 4, 7];
        let k = 5;
        assert_eq!(Solution::length_of_lis(nums, k), 4)
    }

    #[test]
    fn sample3() {
        let nums = vec![1, 5];
        let k = 1;
        assert_eq!(Solution::length_of_lis(nums, k), 1)
    }

    #[test]
    fn issue() {
        let nums = vec![1, 1, 5];
        let k = 3;
        assert_eq!(Solution::length_of_lis(nums, k), 1)
    }
}
