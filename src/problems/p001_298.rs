pub struct Solution;

impl Solution {
    pub fn max_candies(
        mut status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let n = status.len();

        use std::collections::VecDeque;
        let mut que = VecDeque::new();

        let mut found = vec![false; n];
        let mut vis = vec![false; n];

        for x in initial_boxes {
            let x = x as usize;
            found[x] = true;

            if status[x] == 1 && !vis[x] {
                que.push_back(x);
                vis[x] = true;
            }
        }

        let mut ans = 0;
        while let Some(x) = que.pop_front() {
            ans += candies[x];

            for &k in &keys[x] {
                let k = k as usize;
                status[k] = 1;

                if found[k] && !vis[k] {
                    que.push_back(k);
                    vis[k] = true;
                }
            }

            for &y in &contained_boxes[x] {
                let y = y as usize;
                found[y] = true;

                if status[y] == 1 && !vis[y] {
                    que.push_back(y);
                    vis[y] = true;
                }
            }
        }

        ans
    }
}

#[test]
fn test1() {
    let status = vec![1, 0, 1, 0];
    let candies = vec![7, 5, 4, 100];
    let keys = vec![vec![], vec![], vec![1], vec![]];
    let contained_boxes = vec![vec![1, 2], vec![3], vec![], vec![]];
    let initial_boxes = vec![0];

    assert_eq!(
        Solution::max_candies(status, candies, keys, contained_boxes, initial_boxes),
        16
    );
}

#[test]
fn test2() {
    let status = vec![1, 0, 0, 0, 0, 0];
    let candies = vec![1, 1, 1, 1, 1, 1];
    let keys = vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]];
    let contained_boxes = vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]];
    let initial_boxes = vec![0];

    assert_eq!(
        Solution::max_candies(status, candies, keys, contained_boxes, initial_boxes),
        6
    );
}

#[test]
fn test3() {
    let status = vec![1, 1, 1];
    let candies = vec![100, 1, 100];
    let keys = vec![vec![], vec![0, 2], vec![]];
    let contained_boxes = vec![vec![], vec![], vec![]];
    let initial_boxes = vec![1];

    assert_eq!(
        Solution::max_candies(status, candies, keys, contained_boxes, initial_boxes),
        1
    );
}

#[test]
fn test4() {
    let status = vec![1];
    let candies = vec![100];
    let keys = vec![vec![]];
    let contained_boxes = vec![vec![]];
    let initial_boxes = vec![];

    assert_eq!(
        Solution::max_candies(status, candies, keys, contained_boxes, initial_boxes),
        0
    );
}

#[test]
fn test5() {
    let status = vec![1, 1, 1];
    let candies = vec![2, 3, 2];
    let keys = vec![vec![], vec![], vec![]];
    let contained_boxes = vec![vec![], vec![], vec![]];
    let initial_boxes = vec![2, 1, 0];

    assert_eq!(
        Solution::max_candies(status, candies, keys, contained_boxes, initial_boxes),
        7
    );
}
