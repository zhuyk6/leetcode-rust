struct Solution;

type Link = Option<Box<Node>>;

struct Node {
    left: i32,
    right: i32,
    lc: Link,
    rc: Link,
    max: i32,
}

impl Node {
    fn new(left: i32, right: i32) -> Self {
        Node {
            left,
            right,
            lc: None,
            rc: None,
            max: i32::MIN,
        }
    }

    fn update(&mut self) {
        self.max = i32::max(
            self.lc.as_ref().map_or(i32::MIN, |node| node.max),
            self.rc.as_ref().map_or(i32::MIN, |node| node.max),
        );
    }

    #[allow(unused)]
    fn insert(&mut self, pos: i32, val: i32) {
        if self.left == self.right {
            // leaf
            self.max = self.max.max(val);
        } else {
            // internal
            let mid = (self.left + self.right) >> 1;
            if pos <= mid {
                let mut node = self
                    .lc
                    .take()
                    .unwrap_or_else(|| Box::new(Node::new(self.left, mid)));
                node.insert(pos, val);
                self.lc = Some(node);
            } else {
                let mut node = self
                    .rc
                    .take()
                    .unwrap_or_else(|| Box::new(Node::new(mid + 1, self.right)));
                node.insert(pos, val);
                self.rc = Some(node);
            }
            self.update();
        }
    }

    #[allow(unused)]
    fn insert_f(left: i32, right: i32, link: Link, pos: i32, val: i32) -> Link {
        let mut node = link.unwrap_or_else(|| Box::new(Node::new(left, right)));
        if left == right {
            node.max = node.max.max(val);
        } else {
            let mid = (left + right) >> 1;
            if pos <= mid {
                node.lc = Node::insert_f(left, mid, node.lc.take(), pos, val);
            } else {
                node.rc = Node::insert_f(mid + 1, right, node.rc.take(), pos, val);
            }
            node.update();
        }
        Some(node)
    }

    fn query(&self, pos: i32) -> i32 {
        if self.left == self.right {
            return self.max;
        }
        let mid = (self.left + self.right) >> 1;
        if pos <= mid {
            i32::max(
                self.lc.as_ref().map_or(i32::MIN, |node| node.query(pos)),
                self.rc.as_ref().map_or(i32::MIN, |node| node.max),
            )
        } else {
            self.rc.as_ref().map_or(i32::MIN, |node| node.query(pos))
        }
    }
}

const LEFT: i32 = 1;
const RIGHT: i32 = 1_000_000_000;

#[allow(unused)]
struct SegmentTree {
    root: Node,
}

impl SegmentTree {
    #[allow(unused)]
    fn new() -> Self {
        SegmentTree {
            root: Node::new(LEFT, RIGHT),
        }
    }

    #[allow(unused)]
    fn insert(&mut self, pos: i32, val: i32) {
        self.root.insert(pos, val);
    }

    #[allow(unused)]
    fn query(&self, pos: i32) -> i32 {
        self.root.query(pos)
    }
}

#[allow(unused)]
struct SegmentTreeF {
    root: Link,
}

impl SegmentTreeF {
    #[allow(unused)]
    fn new() -> Self {
        SegmentTreeF { root: None }
    }

    #[allow(unused)]
    fn insert(&mut self, pos: i32, val: i32) {
        self.root = Node::insert_f(LEFT, RIGHT, self.root.take(), pos, val);
    }

    #[allow(unused)]
    fn query(&self, pos: i32) -> i32 {
        self.root.as_ref().map_or(i32::MIN, |node| node.query(pos))
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn maximum_sum_queries(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = nums1.len();
        let mut sorted_nums_id: Vec<usize> = (0..n).collect();
        sorted_nums_id.sort_unstable_by(|i, j| match nums1[*i].cmp(&nums1[*j]) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => nums2[*j].cmp(&nums2[*i]),
        });

        let m = queries.len();
        let mut sorted_queries_id: Vec<usize> = (0..m).collect();
        sorted_queries_id.sort_unstable_by(|i, j| queries[*j][0].cmp(&queries[*i][0]));

        let mut ans: Vec<i32> = vec![-1; m];
        let mut tree = SegmentTreeF::new();

        let mut j = 0;
        for i in 0..m {
            while j < n && nums1[sorted_nums_id[j]] >= queries[sorted_queries_id[i]][0] {
                tree.insert(
                    nums2[sorted_nums_id[j]],
                    nums1[sorted_nums_id[j]] + nums2[sorted_nums_id[j]],
                );
                j += 1;
            }
            ans[sorted_queries_id[i]] = match tree.query(queries[sorted_queries_id[i]][1]) {
                i32::MIN => -1,
                v => v,
            }
        }

        ans
    }
}

#[test]
fn test1() {
    let nums1 = vec![4, 3, 1, 2];
    let nums2 = vec![2, 4, 9, 5];
    let queries = [[4, 1], [1, 3], [2, 5]];
    let queries: Vec<Vec<i32>> = queries.into_iter().map(Vec::from).collect();

    assert_eq!(
        Solution::maximum_sum_queries(nums1, nums2, queries),
        vec![6, 10, 7]
    );
}

#[test]
fn test2() {
    let nums1 = vec![3, 2, 5];
    let nums2 = vec![2, 3, 4];
    let queries = [[4, 4], [3, 2], [1, 1]];
    let queries: Vec<Vec<i32>> = queries.into_iter().map(Vec::from).collect();

    assert_eq!(
        Solution::maximum_sum_queries(nums1, nums2, queries),
        vec![9, 9, 9]
    );
}

#[test]
fn test3() {
    let nums1 = vec![2, 1];
    let nums2 = vec![2, 3];
    let queries = [[3, 3]];
    let queries: Vec<Vec<i32>> = queries.into_iter().map(Vec::from).collect();

    assert_eq!(
        Solution::maximum_sum_queries(nums1, nums2, queries),
        vec![-1]
    );
}

#[test]
fn test4() {
    let nums1 = vec![72, 88, 53, 63, 95, 46];
    let nums2 = vec![78, 56, 35, 72, 56, 63];
    let queries = [[86, 86], [24, 8]];
    let queries: Vec<Vec<i32>> = queries.into_iter().map(Vec::from).collect();

    assert_eq!(
        Solution::maximum_sum_queries(nums1, nums2, queries),
        vec![-1, 151]
    );
}
