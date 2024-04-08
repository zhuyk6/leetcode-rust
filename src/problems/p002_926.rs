struct Solution;

struct Node {
    left: usize,
    right: usize,
    lc: Option<Box<Node>>,
    rc: Option<Box<Node>>,
    max: i64,
}

impl Node {
    fn new(left: usize, right: usize) -> Self {
        Node {
            left,
            right,
            lc: None,
            rc: None,
            max: i64::MIN,
        }
    }

    fn build(left: usize, right: usize) -> Self {
        if left == right {
            Node::new(left, right)
        } else {
            let mid = (left + right) >> 1;
            let mut node = Node::new(left, right);
            node.lc = Some(Box::new(Node::build(left, mid)));
            node.rc = Some(Box::new(Node::build(mid + 1, right)));
            node
        }
    }

    fn query(&self, left: usize, right: usize) -> i64 {
        if left <= self.left && self.right <= right {
            self.max
        } else {
            let mut max = i64::MIN;
            let mid = (self.left + self.right) >> 1;
            if left <= mid {
                max = max.max(self.lc.as_ref().unwrap().query(left, right));
            }
            if right > mid {
                max = max.max(self.rc.as_ref().unwrap().query(left, right));
            }
            max
        }
    }

    fn insert(&mut self, pos: usize, val: i64) {
        if self.left == self.right {
            self.max = self.max.max(val);
        } else {
            let mid = (self.left + self.right) >> 1;
            if pos <= mid {
                self.lc.as_mut().unwrap().insert(pos, val);
            } else {
                self.rc.as_mut().unwrap().insert(pos, val);
            }
            self.max = self
                .lc
                .as_ref()
                .into_iter()
                .chain(self.rc.as_ref())
                .map(|node| node.max)
                .max()
                .unwrap();
        }
    }
}

#[allow(unused)]
impl Solution {
    pub fn max_balanced_subsequence_sum(nums: Vec<i32>) -> i64 {
        let n = nums.len();

        let (order, tot) = {
            // v[i] = nums[i] - i
            let mut val_idx: Vec<(usize, i32)> = nums
                .iter()
                .cloned()
                .enumerate()
                .map(|(i, v)| (i, v - i as i32))
                .collect();
            val_idx.sort_unstable_by_key(|(idx, v)| *v);

            let mut iter = val_idx.into_iter();
            let mut tot = 0;
            let mut last = iter.next().unwrap();

            let mut order = vec![usize::MAX; n];
            order[last.0] = tot;

            for cur in iter {
                if cur.1 > last.1 {
                    tot += 1;
                }
                order[cur.0] = tot;
                last = cur;
            }

            (order, tot)
        };

        println!("order: {:?}", order);

        let mut root = Node::build(0, tot);
        let mut ans = i64::MIN;

        for (i, v) in nums.into_iter().enumerate() {
            let mut fi = v as i64;
            let max = root.query(0, order[i]);
            println!("i = {i}, max = {max}");
            if max > 0 {
                fi = max + v as i64;
            }
            ans = ans.max(fi);
            root.insert(order[i], fi);
        }

        ans
    }
}

#[test]
fn test1() {
    let nums = vec![3, 3, 5, 6];
    assert_eq!(Solution::max_balanced_subsequence_sum(nums), 14);
}

#[test]
fn test2() {
    let nums = vec![5, -1, -3, 8];
    assert_eq!(Solution::max_balanced_subsequence_sum(nums), 13);
}

#[test]
fn test3() {
    let nums = vec![-2, -1];
    assert_eq!(Solution::max_balanced_subsequence_sum(nums), -1);
}
